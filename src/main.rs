use chrono::{Local, Timelike};
use std::{
    sync::mpsc::{self, Receiver},
    thread,
    time::{Duration, SystemTime},
};
fn main() {
    let (tx, rx): (mpsc::Sender<String>, Receiver<String>) = mpsc::channel();
    let (tx_up, rx_up): (mpsc::Sender<String>, Receiver<String>) = mpsc::channel();

    let read_thread = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(1));
        let command = "FFDD01EE0B0200006400140032050177".to_string();
        println!("发送数据: {},{}", command, milliseconds());
        tx.send(command).unwrap();
        let rx_str = rx_up.recv().unwrap();
        println!("收到数据: {},{}", rx_str,milliseconds());
    });
    run_serial(rx, tx_up);
    read_thread.join().unwrap();
}
fn milliseconds() -> String {
    let now = Local::now();
    format!(
        "{:02}:{:02}:{:02}.{:03}",
        now.hour(),
        now.minute(),
        now.second(),
        now.nanosecond() / 1_000_000
    )
}
fn run_serial(rx: Receiver<String>, tx: mpsc::Sender<String>) {
    let _ = thread::spawn(move || {
        loop {
            let received = rx.recv().unwrap();
            let received_str = received.as_str();
            match received_str {
                "FFDD01EE0B0200006400140032050177" => {
                    tx.send("FF01XXDD005000000000XX".to_string()).unwrap();
                }
                // 模拟向串口写入数据
                _ => println!("未知数据: {}", received_str),
            }
        }
    });
}
