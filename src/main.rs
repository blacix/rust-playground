use std::time::Duration;
use std::thread;

fn parse_v1(data: Vec<u8>) -> String {
    let parsed_string = String::from_utf8_lossy(&data);
    return parsed_string.to_string();
}

fn parse_v2(data: Vec<u8>) -> String {
    let parsed_string = match String::from_utf8(data) {
        Ok(data)  => data,
        // Err(e) => panic!("Problem parsing data: {:?}", e), // will crash the app
        // Err(e) => "".to_string(), // return empty string
        Err(e) => e.to_string(), // priont the error message
    };
    return parsed_string;
}

fn parse_v3(data: Vec<u8>) -> String {
    let parsed_string = String::from_utf8(data).unwrap_or("could nor parse data".to_string());
    return parsed_string;
}

fn thread_function(mut port: Box<dyn SerialPort> ) {
    let mut serial_buf: Vec<u8> = vec![0; 32];
    while true {
        // let read_len = port.read(serial_buf.as_mut_slice()).unwrap_or(0);
        // if read_len != 0 {
        //     println!("len: {}", read_len);
        //     let string_read = parse_v3(serial_buf[0..read_len].to_vec());
        //     println!("data: {}", string_read);
        // }

        if let Ok(read_len) = port.read(serial_buf.as_mut_slice()) {
            println!("len: {}", read_len);
            let string_read = parse_v3(serial_buf[0..read_len].to_vec());
            println!("data: {}", string_read);
        }

        thread::sleep(Duration::from_millis(10));
    }
}

fn main() {
    let ports = serialport::available_ports().expect("No ports found!");
    let port_count = ports.len();
    println!("found {} ports", port_count);
    for p in ports {
        println!("{}", p.port_name);
    }

    let port = serialport::new("COM14", 115200)
    .timeout(Duration::from_millis(10))
    .open().expect("Failed to open port");

    let handle = thread::spawn(|| {
        thread_function(port);
    });

    handle.join().unwrap();
}
