use std::io::{self, Write};
use std::str;
use std::time::Duration;

use serialport::prelude::*;

fn main() {
    let mut settings = SerialPortSettings::default();
    settings.baud_rate = 115200;
    match serialport::open_with_settings("/dev/ttyUSB0", &settings) {
        Ok(mut port) => {
            match port.try_clone() {
                Ok(mut port1) => {
                    let read = std::thread::spawn(move || loop {
                        let mut buffer: Vec<u8> = vec![0; 1000];
                        match port1.read(buffer.as_mut_slice()) {
                            Ok(t) => io::stdout().write_all(&buffer[..t]).unwrap(),
                            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                            Err(e) => eprintln!("{:?}", e),
                        }
                    });
                    for _ in 0..1 {
                        std::thread::sleep(Duration::from_millis(5000));
                        match port.write("AT+GSN\r".as_bytes()) {
                            Ok(_) => println!("send command AT+GSN"),
                            Err(e) => println!("{}", e),
                        }
                    }
                    read.join().expect("spawn read thread error");
                }
                Err(e) => println!("{}", e),
            };
        }
        Err(_) => println!("open serial port error"),
    };
}
