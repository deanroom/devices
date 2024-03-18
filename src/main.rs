use std::time::Duration;

use serialport;
fn main() {
    read_devices();
    // let port = serialport::new("/dev/ttys011", 9600)
    // .timeout(Duration::from_millis(10))
    // .open().expect("Failed to open port");
}

fn read_devices(){
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }
}

fn read_data(device: &str){
    println!("Reading data from device: {}", device);
}
fn set_data(device: &str){
    println!("Setting data from device: {}", device);
}