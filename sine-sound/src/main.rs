
use eventual::Timer;
use std::time::{Instant, Duration};
use std::thread;
use std::sync::mpsc;

use cpal::{StreamData, UnknownTypeOutputBuffer};
use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};

fn main() {
    // let x: f64 = 6.0;

    // let a = x.tan();
    // let b = x.sin() / x.cos();

    // assert_eq!(a, b);
    time_sequence(10 as u64);
    // handle.join();
    // println!("{:?}", result);
}

fn setup_stream_v2() -> (mpsc::Sender<i32>, thread::JoinHandle<i32>) {
 
    // We want to play a frequency so type is i32
    let (sender, receiver): (std::sync::mpsc::Sender<i32>, std::sync::mpsc::Receiver<i32>) = mpsc::channel();

    // Boilerplate
    let host = cpal::default_host();
    let event_loop = host.event_loop();
    let device = host.default_output_device().expect("no output device available");
    let mut supported_formats_range = device.supported_output_formats()
        .expect("error while querying formats");
    let format = supported_formats_range.next()
        .expect("no supported format?!")
        .with_max_sample_rate();
    let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
    event_loop.play_stream(stream_id).expect("failed to play_stream");

    println!("Sound device: {:?}", device.name());

    // play stuff via a i32 channel
    let handle = thread::spawn( move || {
        event_loop.run(move |stream_id, stream_result| {

            let mut frequency = 0;
            let message = receiver.try_recv();
            //println!("message: {:?}", message);
            if message.is_ok() {
                match message.unwrap() {
                    i => {
                        frequency = i as i16;
                        println!("output sound: {} db ?", frequency)
                    }
                };
            }

            let stream_data = match stream_result {
                Ok(data) => data,
                Err(err) => {
                    eprintln!("an error occurred on stream {:?}: {}", stream_id, err);
                    return;
                }
            };
        
            match stream_data {
                StreamData::Output { buffer: UnknownTypeOutputBuffer::U16(mut buffer) } => {
                    for elem in buffer.iter_mut() {
                        println!("a");
                        *elem = u16::max_value() / 2;
                    }
                },
                StreamData::Output { buffer: UnknownTypeOutputBuffer::I16(mut buffer) } => {
                    for elem in buffer.iter_mut() {
                        *elem = frequency + 1000;
                    }
                },
                StreamData::Output { buffer: UnknownTypeOutputBuffer::F32(mut buffer) } => {
                    for elem in buffer.iter_mut() {
                        // TODO
                        println!("c");
                        *elem = 0.0;
                    }
                },
                _ => (),
            }
        });
   });

   println!("returning value");
   return (sender, handle);
}


fn time_sequence(duration_in_sec: u64) {
    let start = Instant::now();
    let (sender, _) = setup_stream_v2();
    let timer = Timer::new();
    // 1000 is the repeat in ms
    let ticks = timer.interval_ms(500).iter();
    let mut delta = Instant::now().duration_since(start);
    for _ in ticks {
        if delta > Duration::new(duration_in_sec, 0) {
            break;
        }
        let delta_u64 = delta.as_secs() as f64;
        let sine = (sine_calculator(delta_u64) + 1.6 as f64) * 100 as f64;
        delta = Instant::now().duration_since(start);
        let _result = sender.send(sine as i32);
    }
}

fn sine_calculator(current: f64) -> f64 {
    return current.sin();
}
