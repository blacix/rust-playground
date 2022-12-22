mod serial_io;
mod ble;


#[tokio::main(flavor = "current_thread")]
async fn main() -> bluer::Result<()> {
    // serial_io::main_serial_io();
    let res = ble::ble_main().await;
    res
}

// fn main() {
//     
// }
