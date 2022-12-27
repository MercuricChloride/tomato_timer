use rodio::source::{SineWave, Source};
use rodio::{OutputStream, Sink};
use std::time::Duration;

pub fn finish_sound() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let beep = SineWave::new(440.0)
        .take_duration(Duration::from_secs_f32(0.25))
        .amplify(0.20);
    let pause = SineWave::new(0.0)
        .take_duration(Duration::from_secs_f32(0.25))
        .amplify(0.20);

    for _ in 0..3 {
        sink.append(beep.clone());
        sink.append(pause.clone());
    }

    sink.sleep_until_end();
}

pub fn start_sound() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let beep = SineWave::new(1000.0)
        .take_duration(Duration::from_secs_f32(0.5))
        .amplify(0.20);

    sink.append(beep.clone());
    sink.sleep_until_end();
}
