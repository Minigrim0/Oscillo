use rodio::source::{SineWave, Source};
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

fn main() {
    // _stream must live as long as the sink
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Add a dummy source of the sake of the example.
    let source = SineWave::new(440.0)
        .take_duration(Duration::from_secs_f32(100.0))
        .amplify(0.20);
    sink.append(source);

    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    sink.sleep_until_end();
}
