use std::io::Write;
use std::time::Duration;
use std::thread::sleep;
use console::Term;
use nb_node::Node;
use serialport::prelude::*;

fn main() {
    let term = Term::stdout();
    term.set_title("Detecting NB-iot signal RSSI");
    term.write_line("****** 检测 NB-iot 信号强度 ******").unwrap();
    write!(&term, "Please set serial port (设置串口): ").unwrap();
    let serial_dev = term.read_line_initial_text("COM1").unwrap();
    write!(&term, "\n").unwrap();
    term.write_line("Detecting...").unwrap();

    let mut settings: SerialPortSettings = Default::default();
    settings.baud_rate = 115200;

    match serialport::open_with_settings(&serial_dev, &settings) {
        Ok(port) => match Node::new(port) {
            Ok(mut node) => {
                println!("create NB module succeed! imei: {}", node.get_imei());
                
                loop {
                    match node.rssi() {
                        Ok(rssi) => println!("RSSI: {}", rssi),
                        Err(()) => println!("get rssi failed"),
                    }
                    sleep(Duration::from_secs(1));
                }

            }
            Err(_) => println!("creat nb node error"),
        },
        Err(_) => println!("open serial port error"),
    }
}
