extern crate cpal;
extern crate futures;
extern crate time;
extern crate notesender;

use futures::stream::Stream;
use futures::task;
use futures::task::Executor;
use futures::task::Run;

use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;


struct MyExecutor;

impl Executor for MyExecutor {
    fn execute(&self, r: Run) {
        r.run();
    }
}


/// Iterator for iterating over Synthesizer samples.
///
/// It is possible to request a SampleIter from a Synthesizer instance.
/// This SampleIter instance will hold a mutable reference to the Synthesizer
/// and in each iteration get the next sample.
///
/// Please note that while a SampleIter exists for a Synthesizer, the state
/// of that synthesizer can not be chaned.
struct SampleIter<'a> {
    synth: &'a mut Synthesizer
}

impl<'a> Iterator for SampleIter<'a> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        return Some(self.synth.next_sampe())
    }
}


/// The Synthesizer itself.
///
/// Holds all the necessary data to generate the sound samples from the playing notes.
struct Synthesizer {
    sample_rate: u32,
    channel_count: u8,
    curr_freq: f64,
    curr_sample: u64,
    volume: f32
}

impl Synthesizer {
    fn new() -> Synthesizer {
        return Synthesizer {sample_rate: 44100, channel_count: 2, curr_freq: 440.0, curr_sample: 0, volume: 0.1}
    }

    /// Gets an iterator that gives the next sample in each iteration.
    /// (each iteration calls next_sample)
    fn samples_iter(&mut self) -> SampleIter {
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
    fn next_sampe(&mut self) -> f32 {
        use std::f64::consts::PI;
        let curr_sec = self.curr_sample as f64 / self.sample_rate as f64;
        self.curr_sample += 1;
        return self.volume * (PI * 2.0 * curr_sec * self.curr_freq).sin() as f32;
    }
}



fn main() {
    playsound();
    
}


fn playsound(){
    let endpoint = cpal::get_default_endpoint().expect("Failed to get default endpoint");
    let format = endpoint.get_supported_formats_list().unwrap().next().expect("Failed to get endpoint format");

    let event_loop = cpal::EventLoop::new();
    let executor = Arc::new(MyExecutor);

    let curr_time = time::get_time();
    println!("Time is: {}", curr_time.sec);

    let (mut voice, stream) = cpal::Voice::new(&endpoint, &format, &event_loop).expect("Failed to create a voice");

    let synth_arc_mtx = Arc::new(Mutex::new(Synthesizer::new()));
    {
        let mut synth = synth_arc_mtx.lock().unwrap();
        synth.sample_rate = format.samples_rate.0;
        synth.channel_count = format.channels.len() as u8;
    } // mutex gets unlocked here

    voice.play();
    // creating a new reference counted reference to synth
    let buffer_thread_synth = synth_arc_mtx.clone();
    task::spawn(stream.for_each(move |buffer| -> Result<_, ()> {
        let mut synth = buffer_thread_synth.lock().unwrap();
        match buffer {
            cpal::UnknownTypeBuffer::U16(mut buffer) => {
                for (sample, value) in buffer.chunks_mut(synth.channel_count as usize).zip(synth.samples_iter()) {
                    let value = ((value * 0.5 + 0.5) * std::u16::MAX as f32) as u16;
                    for out in sample.iter_mut() { *out = value; }
                }
            },

            cpal::UnknownTypeBuffer::I16(mut buffer) => {
                for (sample, value) in buffer.chunks_mut(synth.channel_count as usize).zip(synth.samples_iter()) {
                    let value = (value * std::i16::MAX as f32) as i16;
                    for out in sample.iter_mut() { *out = value; }
                }
            },

            cpal::UnknownTypeBuffer::F32(mut buffer) => {
                for (sample, value) in buffer.chunks_mut(synth.channel_count as usize).zip(synth.samples_iter()) {
                    for out in sample.iter_mut() { *out = value; }
                }
            },
        };

        Ok(())
    })).execute(executor);

	notesender::note_to_freq("A2");
    notesender::init_notes();

    let freqs = [
        440.0,
        466.1637615180899,
        493.8833012561241,
        523.2511306011972,
        554.3652619537442,
        587.3295358348151,
        622.2539674441618,
        659.2551138257398,
        698.4564628660078,
        739.9888454232688,
        783.9908719634985,
        830.6093951598903,
    ];
    let mut curr_freq_id = 0;

    // creating a new reference counted reference to synth
    let control_thread_synth = synth_arc_mtx.clone();
    thread::spawn(move || {
        loop {
            {
                let mut synth = control_thread_synth.lock().unwrap();
                synth.curr_freq = freqs[curr_freq_id];
            }
            voice.play();
            curr_freq_id = (curr_freq_id+1) % freqs.len();
            if curr_freq_id == 0 {
                voice.pause();
            }
            thread::sleep(Duration::from_millis(500));
        }
    });


    event_loop.run();
}
