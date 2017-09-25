extern crate rodio;

use rodio::Sink;
use rodio::Source;

use std::time::Duration;

#[test]
fn test() {
    let endpoints = rodio::get_endpoints_list();
    let mut endpoint = rodio::get_default_endpoint().unwrap();
    for e in endpoints {
        println!("{}", e.get_name());
        if e.get_name() == "pulse" {
            endpoint = e;
        }
    }
    let mut sink = Sink::new(&endpoint);

    let source = Blorp { glorp: 0 };
    sink.append(source);
    sink.set_volume(0.7);
    std::thread::sleep_ms(8000);
}

struct Blorp {
    glorp: usize,
}

impl Iterator for Blorp {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.glorp += 1;
        let glob = (self.glorp as f32 / 48000.0 + 1.0) * 200.0;
        Some((((self.glorp % glob as usize) as f32 / glob as f32).sqrt() - 0.5) * 2.0)
    }
}

impl Source for Blorp {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn samples_rate(&self) -> u32 {
        48000
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}