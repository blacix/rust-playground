mod serial_io;
mod ble;

// fn main() {
//     serial_io::main_serial_io();
// }

#[tokio::main(flavor = "current_thread")]
async fn main() -> bluer::Result<()> {
    ble::ble_main().await
}
