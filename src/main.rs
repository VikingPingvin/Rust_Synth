extern crate cpal;
extern crate futures;
extern crate time;
extern crate notesender;
extern crate synth;

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

    let synth_arc_mtx = Arc::new(Mutex::new(synth::Synthesizer::new()));
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


    voice.play();
    // creating a new reference counted reference to synth
    let control_thread_synth = synth_arc_mtx.clone();
    thread::spawn(move || {
        loop {
            
            {
                let mut synth = control_thread_synth.lock().unwrap();
                synth.play_note(synth::midi(60));
                synth.play_note(synth::midi(64));
            }
            
            thread::sleep(Duration::from_millis(250));
            
            {
                let mut synth = control_thread_synth.lock().unwrap();
                synth.stop_note(synth::midi(60));
                synth.stop_note(synth::midi(64));
            }
            thread::sleep(Duration::from_millis(250));
            

            {
                let mut synth = control_thread_synth.lock().unwrap();
                synth.play_note(synth::E4);
            }
            thread::sleep(Duration::from_millis(250));
            {
                let mut synth = control_thread_synth.lock().unwrap();
                synth.stop_note(synth::E4);
            }
            thread::sleep(Duration::from_millis(250));

            //------------------------

            {
                let mut synth = control_thread_synth.lock().unwrap();
                synth.play_note(synth::C4);
            }
            thread::sleep(Duration::from_millis(250));
            {
                let mut synth = control_thread_synth.lock().unwrap();
                synth.stop_note(synth::C4);
            }
            thread::sleep(Duration::from_millis(250));

            {
                let mut synth = control_thread_synth.lock().unwrap();
                synth.play_note(synth::E4);
            }
            thread::sleep(Duration::from_millis(250));
            {
                let mut synth = control_thread_synth.lock().unwrap();
                synth.stop_note(synth::E4);
            }
            thread::sleep(Duration::from_millis(250));


            // =============================================

            {
                let mut synth = control_thread_synth.lock().unwrap();
                synth.play_note(synth::G4);
            }
            thread::sleep(Duration::from_millis(500));
            {
                let mut synth = control_thread_synth.lock().unwrap();
                synth.stop_note(synth::G4);
            }
            thread::sleep(Duration::from_millis(500));

            {
                let mut synth = control_thread_synth.lock().unwrap();
                synth.play_note(synth::G4);
            }
            thread::sleep(Duration::from_millis(500));
            {
                let mut synth = control_thread_synth.lock().unwrap();
                synth.stop_note(synth::G4);
            }
            thread::sleep(Duration::from_millis(500));
            
        }
    });


    event_loop.run();
}
