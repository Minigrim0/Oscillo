use rodio::buffer::SamplesBuffer;
use rodio::{OutputStream, Sink};
use std::f32::consts::PI;
use std::time::Duration;
use std::time::Instant;

struct Segment {
    from_value: f32,
    to_value: f32,
    duration: i32, // frames
}

impl Segment {
    pub fn new(from_value: f32, to_value: f32, duration: i32) -> Self {
        Self {
            from_value,
            to_value,
            duration,
        }
    }

    pub fn get_at(&self, sample: i32) -> f32 {
        if sample < self.duration {
            return self.from_value
                + (self.to_value - self.from_value) * (sample as f32 / self.duration as f32);
        } else {
            return self.to_value;
        }
    }
}

fn three() -> (Vec<Segment>, Vec<Segment>) {
    (
        vec![
            Segment::new(0.5, -0.5, 5),
            // Start of stuff
            Segment::new(0.0, 0.1, 5),
            Segment::new(0.1, 0.2, 10),
            Segment::new(0.2, 0.3, 20),
            Segment::new(0.3, 0.4, 35),
            Segment::new(0.4, 0.5, 45),
            Segment::new(0.5, 0.5, 20),
            Segment::new(-0.5, -0.4, 45),
            Segment::new(-0.4, -0.3, 35),
            Segment::new(-0.3, -0.2, 20),
            Segment::new(-0.2, -0.1, 10),
            Segment::new(-0.1, 0.0, 5),
            Segment::new(-0.0, -0.1, 5),
            Segment::new(-0.1, -0.2, 10),
            Segment::new(-0.2, -0.3, 20),
            Segment::new(-0.3, -0.4, 35),
            Segment::new(-0.4, -0.5, 45),
            Segment::new(-0.5, -0.5, 20),
            Segment::new(-0.5, -0.4, 45),
            Segment::new(-0.4, -0.3, 35),
            Segment::new(-0.3, -0.2, 20),
            Segment::new(-0.2, -0.1, 10),
            Segment::new(-0.1, -0.0, 5),
        ],
        vec![
            Segment::new(0.5, -0.5, 5),
            // Start of stuff
            Segment::new(-0.0, -0.1, 5),
            Segment::new(-0.1, -0.2, 10),
            Segment::new(-0.2, -0.3, 20),
            Segment::new(-0.3, -0.4, 35),
            Segment::new(-0.4, -0.5, 45),
            Segment::new(-0.5, -0.5, 20),
            Segment::new(-0.5, -0.4, 45),
            Segment::new(-0.4, -0.3, 35),
            Segment::new(-0.3, -0.2, 20),
            Segment::new(-0.2, -0.1, 10),
            Segment::new(-0.1, -0.0, 5),
            Segment::new(-0.0, -0.1, 5),
            Segment::new(-0.1, -0.2, 10),
            Segment::new(-0.2, -0.3, 20),
            Segment::new(-0.3, -0.4, 35),
            Segment::new(-0.4, -0.5, 45),
            Segment::new(-0.5, -0.5, 20),
            Segment::new(0.5, 0.5, 20),
            Segment::new(0.5, 0.4, 45),
            Segment::new(0.4, 0.3, 35),
            Segment::new(0.3, 0.2, 20),
            Segment::new(0.2, 0.1, 10),
            Segment::new(0.1, 0.0, 5),
        ],
    )
}

fn letter_c() -> (Vec<Segment>, Vec<Segment>) {
    (
        vec![
            Segment::new(0.5, -0.5, 5),
            // Start of stuff
            Segment::new(0.0, 0.0, 50),
            Segment::new(0.0, 0.1, 5),
            Segment::new(0.1, 0.2, 10),
            Segment::new(0.2, 0.3, 20),
            Segment::new(0.3, 0.4, 30),
            Segment::new(0.4, 0.5, 40),
            Segment::new(0.5, 0.5, 150),
            Segment::new(-0.5, -0.4, 45),
            Segment::new(-0.4, -0.3, 35),
            Segment::new(-0.3, -0.2, 20),
            Segment::new(-0.2, -0.1, 10),
            Segment::new(-0.1, -0.0, 5),
        ],
        vec![
            Segment::new(0.5, -0.5, 5),
            // Start of stuff
            Segment::new(0.0, 0.0, 50),
            Segment::new(-0.0, -0.1, 5),
            Segment::new(-0.1, -0.2, 10),
            Segment::new(-0.2, -0.3, 20),
            Segment::new(-0.3, -0.4, 35),
            Segment::new(-0.4, -0.5, 45),
            Segment::new(0.5, 0.5, 150),
            Segment::new(0.5, 0.5, 20),
            Segment::new(0.5, 0.4, 45),
            Segment::new(0.4, 0.3, 35),
            Segment::new(0.3, 0.2, 20),
            Segment::new(0.2, 0.1, 10),
            Segment::new(0.1, 0.0, 5),
        ],
    )
}

fn eight() -> (Vec<Segment>, Vec<Segment>) {
    (
        vec![
            Segment::new(0.5, -0.5, 5),
            // Start of stuff
            Segment::new(0.0, 0.1, 5),
            Segment::new(0.1, 0.2, 10),
            Segment::new(0.2, 0.3, 20),
            Segment::new(0.3, 0.4, 35),
            Segment::new(0.4, 0.5, 45),
            Segment::new(0.5, 0.5, 20),
            Segment::new(0.5, 0.4, 45),
            Segment::new(0.4, 0.3, 35),
            Segment::new(0.3, 0.2, 20),
            Segment::new(0.2, 0.1, 10),
            Segment::new(0.1, 0.0, 5),
            Segment::new(-0.0, -0.1, 5),
            Segment::new(-0.1, -0.2, 10),
            Segment::new(-0.2, -0.3, 20),
            Segment::new(-0.3, -0.4, 35),
            Segment::new(-0.4, -0.5, 45),
            Segment::new(-0.5, -0.5, 20),
            Segment::new(-0.5, -0.4, 45),
            Segment::new(-0.4, -0.3, 35),
            Segment::new(-0.3, -0.2, 20),
            Segment::new(-0.2, -0.1, 10),
            Segment::new(-0.1, -0.0, 5),
        ],
        vec![
            Segment::new(0.5, -0.5, 5),
            // Start of stuff
            Segment::new(0.0, -0.1, 5),
            Segment::new(-0.1, -0.2, 10),
            Segment::new(-0.2, -0.3, 20),
            Segment::new(-0.3, -0.4, 35),
            Segment::new(-0.4, -0.5, 45),
            Segment::new(-0.5, -0.5, 20),
            Segment::new(-0.5, -0.4, 45),
            Segment::new(-0.4, -0.3, 35),
            Segment::new(-0.3, -0.2, 20),
            Segment::new(-0.2, -0.1, 10),
            Segment::new(-0.1, 0.0, 5),
            Segment::new(0.0, 0.1, 5),
            Segment::new(0.1, 0.2, 10),
            Segment::new(0.2, 0.3, 20),
            Segment::new(0.3, 0.4, 35),
            Segment::new(0.4, 0.5, 45),
            Segment::new(0.5, 0.5, 20),
            Segment::new(0.5, 0.4, 45),
            Segment::new(0.4, 0.3, 35),
            Segment::new(0.3, 0.2, 20),
            Segment::new(0.2, 0.1, 10),
            Segment::new(0.1, 0.0, 5),
        ],
    )
}

fn sine() -> (Vec<Segment>, Vec<Segment>) {
    let mut vec1 = vec![];
    let mut vec2 = vec![];

    for x in 1..250 {
        let v: f32 = (x as f32 / 250.0) * 2.0 * PI * 2.0;
        vec1.push(Segment::new((v - 1.0).sin() * 0.5, v.sin() * 0.5, 1));
    }

    for x in 250..500 {
        let v: f32 = ((x + 125) as f32 / 250.0) * 2.0 * PI * 2.0;
        vec2.push(Segment::new((v - 1.0).sin() * 0.5, v.sin() * 0.5, 1));
    }

    (vec1, vec2)
}

pub fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let sample_rate: u32 = 48000; // 44100 Hz

    // let (channel_1, channel_2) = three();
    let channels = 2;

    let mut start = Instant::now();
    let mut total_samples = 0;
    loop {
        let time = (total_samples as f32 / channels as f32) / sample_rate as f32;
        let (channel_1, channel_2): (Vec<Segment>, Vec<Segment>) = if time < 0.5 {
            three()
        } else if time < 1.0 {
            eight()
        } else if time < 1.5 {
            letter_c()
        } else if time < 2.0 {
            three()
        } else if time < 3.0 {
            sine()
        } else {
            while start.elapsed() < Duration::from_millis(3500) {}
            start = Instant::now();
            total_samples = 0;
            (vec![], vec![])
        };

        let mut source1 = vec![];
        for segment in channel_1.iter() {
            for i in 0..segment.duration {
                let val = segment.get_at(i);
                source1.push(val);
            }
        }

        let mut source2 = vec![];
        for segment in channel_2.iter() {
            for i in 0..segment.duration {
                let val = segment.get_at(i);
                source2.push(val);
            }
        }

        let sources = source1
            .into_iter()
            .zip(source2)
            .collect::<Vec<(f32, f32)>>();
        let mut source = sources
            .iter()
            .flat_map(|(a, b)| vec![*a, *b])
            .collect::<Vec<f32>>();
        // let mut source = vec![];

        // Pad
        for i in source.len()..(545 * channels as usize) {
            // if i < (545 * channels as usize / 2) {
            //     source.push(-1.0);
            // } else {
            //     source.push(1.0);
            // }
            source.push(-0.5);
        }

        total_samples += source.len() as u32;
        sink.append(SamplesBuffer::new(
            channels as u16,
            sample_rate,
            source.iter().map(|n| *n).collect::<Vec<f32>>(),
        ));
    }
}
