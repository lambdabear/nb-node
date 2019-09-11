use nb_node::Node;
use serialport::prelude::*;

fn main() {
    let mut settings: SerialPortSettings = Default::default();
    settings.baud_rate = 115200;

    match serialport::open_with_settings("/dev/ttyUSB0", &settings) {
        Ok(port) => match Node::new(port) {
            Ok(node) => println!("imei: {}", node.get_imei()),
            Err(_) => println!("creat nb node error"),
        },
        Err(_) => println!("open serial port error"),
    }
}
