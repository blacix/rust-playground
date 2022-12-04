use serialport::{SerialPort, SerialPortInfo};
use std::thread;
use std::thread::{JoinHandle};
use std::time::Duration;
use std::sync::mpsc::{Receiver};

pub enum ThreadMessage {
    NoOp,
    Exit,
}

const SERIAL_THREAD_SLEEP_MS: u64 = 10;
const SERIAL_BUFFER_SIZE: usize = 128;

fn bytes_to_string(data: Vec<u8>) -> String {
    let parsed_string = String::from_utf8(data).unwrap_or("could not parse data".to_string());
    return parsed_string;
}

pub fn serial_thread_function(mut port: Box<dyn SerialPort>, channel: Receiver<ThreadMessage> ) {
    let mut serial_buf: Vec<u8> = vec![0; SERIAL_BUFFER_SIZE];
    let mut running = true;

    while running {
        if let Ok(read_len) = port.read(serial_buf.as_mut_slice()) {
            println!("len: {}", read_len);
            let string_read = bytes_to_string(serial_buf[0..read_len].to_vec());
            print!("{string_read}");
        }

        let message_received = channel.try_recv().unwrap_or(ThreadMessage::NoOp);
        if matches!(message_received, ThreadMessage::Exit) {
            running = false;
        } else {
            thread::sleep(Duration::from_millis(SERIAL_THREAD_SLEEP_MS));
        }
    }
}


// fn bytes_to_string_v2(data: Vec<u8>) -> String {
//     let parsed_string = String::from_utf8_lossy(&data);
//     return parsed_string.to_string();
// }

// fn bytes_to_string_v3(data: Vec<u8>) -> String {
//     let parsed_string = match String::from_utf8(data) {
//         Ok(data)  => data,
//         // Err(e) => panic!("Problem parsing data: {:?}", e), // will crash the app
//         // Err(e) => "".to_string(), // return empty string
//         Err(e) => e.to_string(), // priont the error message
//     };
//     return parsed_string;
// }