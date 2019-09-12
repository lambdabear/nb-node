use nb_node::Node;
use serialport::prelude::*;

fn main() {
    let mut settings: SerialPortSettings = Default::default();
    settings.baud_rate = 115200;

    match serialport::open_with_settings("/dev/ttyUSB0", &settings) {
        Ok(port) => match Node::new(port) {
            Ok(mut node) => {
                println!("imei: {}", node.get_imei());
                // node.register("221.229.214.202", 5683, 86400)
                //     .expect("register failed");
                node.send("0000").expect("send message error");
            }
            Err(_) => println!("creat nb node error"),
        },
        Err(_) => println!("open serial port error"),
    }
}
