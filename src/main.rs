use std::time::Duration;
use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};
use serialport;

mod serial_io;


fn main() {
    // let s: serial_io::SerialIO;

    // list available ports
    let ports = serialport::available_ports().expect("No ports found!");
    // reference must be used on ports because iterator changes the collection
    // and ports.get won't compile
    for (i, p) in ports.iter().enumerate() {
        println!("{} - {}", i + 1, p.port_name);
    }


    // let the user select a port
    println!("pick a port:");

    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("cannot read input");
    // parse template param can be omitted if variable type is explicitly declared
    // let port_index: usize = user_input.trim().parse::<usize>().expect("type a number between 1< ");
    let port_index = match user_input.trim().parse::<usize>() {
        Ok(i)  => {
            if i > 0 && i <= ports.len() {
                i - 1
            } else {
                panic!("Invalid port selected: {}", user_input.trim())
            }
        },
        Err(e) => panic!("Problem parsing data: {:?}", e.to_string())
    };

    let selected_port_info = ports.get(port_index).unwrap();
    // TODO config from user input
    let baud_rate = 115200;
    let flow_control = serialport::FlowControl::None;
    let parity = serialport::Parity::None;
    let data_bits = serialport::DataBits::Eight;
    
    // open selected serial port
    let port = serialport::new(selected_port_info.port_name.to_string(), baud_rate)
    .timeout(Duration::from_millis(10))
    .flow_control(flow_control)
    .parity(parity)
    .data_bits(data_bits)
    .open().expect("Failed to open port");

    println!("port {} open", selected_port_info.port_name.to_string());
    println!("press enter to exit");


    // starting thread to read serial port
    let (channel_main_thread, channel_serial_therad) = channel();
    // TODO move or not move?
    let serial_thread_handle = thread::spawn(move || {
        serial_io::serial_thread_function(port, channel_serial_therad);
    });

    // reading user input
    std::io::stdin().read_line(&mut user_input).expect("cannot read input");
    channel_main_thread.send(serial_io::ThreadMessage::Exit).unwrap();
    serial_thread_handle.join().unwrap();
}
