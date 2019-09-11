use std::io;
use std::str;
use std::time::Duration;

use serialport::prelude::*;

fn at_command<'a>(port: &mut Box<dyn serialport::SerialPort>, cmd: &'a str) -> Result<String, ()> {
    let mut buffer: Vec<u8> = vec![0; 100];
    let mut res: Vec<u8> = vec![];
    match port.write(cmd.as_bytes()) {
        Ok(_) => {
            for _ in 0..100 {
                std::thread::sleep(Duration::from_millis(10));
                match port.read(buffer.as_mut_slice()) {
                    Ok(t) => {
                        let mut cache = buffer.clone();
                        cache.truncate(t);
                        res.append(&mut cache);
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(_) => return Err(()),
                }
            }
            match str::from_utf8(&res) {
                Ok(res) => Ok(res.to_owned()),
                Err(_) => Err(()),
            }
        }
        Err(_) => Err(()),
    }
}

pub struct Node {
    port: Box<dyn serialport::SerialPort>,
    imei: String,
    // operator: String,
}

impl Node {
    pub fn new(mut port: Box<dyn serialport::SerialPort>) -> Result<Self, ()> {
        std::thread::sleep(Duration::from_millis(1000));
        match at_command(&mut port, "AT+GSN\n") {
            Ok(s) => {
                println!("{}", s);
                let mut lines = s.lines();
                lines.next();
                match lines.next() {
                    Some(imei) => Ok(Node {
                        port,
                        imei: imei.to_string(),
                    }),
                    None => Err(()),
                }
            }
            Err(_) => Err(()),
        }
    }

    pub fn get_imei(&self) -> &str {
        &self.imei
    }
}
