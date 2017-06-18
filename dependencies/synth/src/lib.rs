
use std::collections::HashMap;

/// Iterator for iterating over Synthesizer samples.
///
/// It is possible to request a SampleIter from a Synthesizer instance.
/// This SampleIter instance will hold a mutable reference to the Synthesizer
/// and in each iteration get the next sample.
///
/// Please note that while a SampleIter exists for a Synthesizer, the state
/// of that synthesizer can not be chaned.
pub struct SampleIter<'a> {
    synth: &'a mut Synthesizer
}

impl<'a> Iterator for SampleIter<'a> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        return Some(self.synth.next_sampe())
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct Note {
    midi_id: u8 // represents a node by its midi note number
}

pub fn midi(id:u8) -> Note {
    return Note{midi_id:id}
}

pub const C4: Note = Note {midi_id: 60};
pub const E4: Note = Note {midi_id: 64};
pub const G4: Note = Note {midi_id: 67};

enum NoteEnvelope {
    //TODO: Add Decay (ADSR = Attack-Decay-Sustain-Release) ?
    Attack,
    Sustain,
    Release
}

struct NoteState {
    intensity: f32,
    envelope: NoteEnvelope,
}

/*
struct ActiveNote {
    note: Note,
    note_state: NoteState,
}
*/

/// The Synthesizer itself.
///
/// Holds all the necessary data to generate the sound samples from the playing notes.
pub struct Synthesizer {
    // TODO consider removing the public specifiers
    pub sample_rate: u32,
    pub channel_count: u8,
    //pub curr_freq: f64,
    pub volume: f32,

    attack_time: f64,
    release_time: f64,

    curr_sample: u64,
    active_notes: HashMap<Note, NoteState>,

    note_freqs: [f64; 127],
}

impl Synthesizer {
    pub fn new() -> Synthesizer {
        let mut result = Synthesizer {
            sample_rate: 44100,
            channel_count: 2,
            //curr_freq: 440.0,
            curr_sample: 0,
            volume: 0.1,
            attack_time: 0.002,
            release_time: 0.2,
            note_freqs: [0.0; 127],
            active_notes: HashMap::new(),
        };

        let a = 440.0;
        for i in 0..result.note_freqs.len() {
            result.note_freqs[i] = (a/32.0) * 2f64.powf((i as f64-9.0) / 12.0);
        }

        return result;
    }

    pub fn play_note(&mut self, note: Note) {
        self.active_notes.insert(note, NoteState{ intensity: 0.0, envelope: NoteEnvelope::Attack});
    }

    pub fn stop_note(&mut self, note: Note) {
        if let Some(ref mut note_state) = self.active_notes.get_mut(&note) {
            note_state.envelope = NoteEnvelope::Release;
        }
    }

    /// Gets an iterator that gives the next sample in each iteration.
    /// (each iteration calls next_sample)
    pub fn samples_iter(&mut self) -> SampleIter {
        return SampleIter {synth: self}
    }

    /// Gets the next sample in the audio signal.
    ///
    /// This function will return a value in range [-1, 1] denoting the required
    /// position of the membrane in order to play the sound. The corresponding time point is
    /// calculated from the sample rate of the synthesizer.
    ///
    /// Each call will step the synthesizer to the following sample so that consecutive calls
    /// return consecutive samples.
    pub fn next_sampe(&mut self) -> f32 {
        use std::f64::consts::PI;
        let delta_t = 1.0 / self.sample_rate as f64;
        let curr_sec = self.curr_sample as f64 / self.sample_rate as f64;
        self.curr_sample += 1;
        let mut retval = 0.0;

        for mut curr_pair in self.active_notes.iter_mut() {
            let ref mut curr_note = curr_pair.1;
            match curr_note.envelope {
                NoteEnvelope::Attack => {
                    curr_note.intensity += ((delta_t / self.attack_time) as f32).min(1.0);
                    if curr_note.intensity >= 1.0 {
                        curr_note.envelope = NoteEnvelope::Sustain;
                    }
                }
                NoteEnvelope::Release => {
                    curr_note.intensity -= ((delta_t / self.release_time) as f32).max(0.0);
                }
                _ => {}
            }

            let freq = self.note_freqs[curr_pair.0.midi_id as usize];
            retval += self.volume * curr_note.intensity * (PI * 2.0 * curr_sec * freq).sin() as f32;
        }

        // remove finished notes (all notes with decay completeness >= 1.0)
        self.active_notes.retain(|_, ref note_state| {
            return
                match note_state.envelope {
                    NoteEnvelope::Release => {
                        note_state.intensity > 0.0
                    }
                    _ => true
                };
        });

        return retval;
    }
}


