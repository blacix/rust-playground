use std::time::Duration;
use std::thread;

fn main() {
    let ports = serialport::available_ports().expect("No ports found!");
    let port_count = ports.len();
    println!("found {} ports", port_count);
    for p in ports {
        println!("{}", p.port_name);
    }

    let mut port = serialport::new("COM6", 115200)
    .timeout(Duration::from_millis(10))
    .open().expect("Failed to open port");

    let mut serial_buf: Vec<u8> = vec![0; 32];
    while true {
        if let Ok(read_len) = port.read(serial_buf.as_mut_slice()) {
            println!("len: {}", read_len);
            let s = String::from_utf8_lossy(&serial_buf);
            println!("data: {}", s.to_string());
        }
        thread::sleep(Duration::from_millis(10));
    }
}
