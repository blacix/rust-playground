use std::time::Duration;
use std::thread;
use std::sync::mpsc::{channel, Receiver};

use serialport::SerialPort;

enum ThreadMessage {
    NoOp,
    Exit,
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

fn bytes_to_string(data: Vec<u8>) -> String {
    let parsed_string = String::from_utf8(data).unwrap_or("could not parse data".to_string());
    return parsed_string;
}

fn serial_thread_function(mut port: Box<dyn SerialPort>, channel: Receiver<ThreadMessage> ) {
    let mut serial_buf: Vec<u8> = vec![0; 32];
    let mut running = true;
    while running {
        if let Ok(read_len) = port.read(serial_buf.as_mut_slice()) {
            // println!("len: {}", read_len);
            let string_read = bytes_to_string(serial_buf[0..read_len].to_vec());
            print!("{string_read}");
        }

        let message_received = channel.try_recv().unwrap_or(ThreadMessage::NoOp);
        if matches!(message_received, ThreadMessage::Exit) {
            running = false;
        } else {
            thread::sleep(Duration::from_millis(10));
        }
    }
}

fn main() {
    // list available ports
    let ports = serialport::available_ports().expect("No ports found!");
    let mut cnt = 1;
    // reference must be used on ports because iterator changes the collection
    // and ports.get won't compile
    for p in &ports {
        println!("{} - {}", cnt, p.port_name);
        cnt += 1;
    }


    // let the user select a port
    println!("pick a port:");

    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("cannot read input");
    let port_index = match user_input.trim().parse::<usize>() {
        Ok(i)  => i - 1,
        Err(e) => panic!("Problem parsing data: {:?}", e.to_string())
    };
    
    let selected_port_info = ports.get(port_index).unwrap();
    println!("selected port: {}", selected_port_info.port_name.to_string());


    // open selected serial port
    let port = serialport::new(selected_port_info.port_name.to_string(), 115200)
    .timeout(Duration::from_millis(10))
    .open().expect("Failed to open port");

    println!("port {} open", selected_port_info.port_name.to_string());
    println!("press enter to exit");


    // starting thread to read serial port
    let (channel_main_thread, channel_serial_therad) = channel();
    let serial_thread_handle = thread::spawn(|| {
        serial_thread_function(port, channel_serial_therad);
    });

    // reading user input
    std::io::stdin().read_line(&mut user_input).expect("cannot read input");
    channel_main_thread.send(ThreadMessage::Exit).unwrap();
    serial_thread_handle.join().unwrap();
}
