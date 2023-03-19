// Create a node module that can be used to create a new note that can
// later be reproduced by rodio

use rodio::{source::SineWave, Sink, Source};

pub struct Note {
    pub freq: f32,
    pub duration: f32,
}

impl Note {
    pub fn new(freq: f32, duration: f32) -> Note {
        Note { freq, duration }
    }

    pub fn play(&self) {
        // get default output device from rodio
        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&handle);
        let source = SineWave::new(440.0).amplify(0.20);

        match sink {
            Ok(sink) => {
                sink.append(source);
                sink.sleep_until_end();
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
