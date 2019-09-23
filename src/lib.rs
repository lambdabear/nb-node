use std::io;
use std::str;
use std::time::Duration;

// use serialport::prelude::*;

fn at_command<'a>(port: &mut Box<dyn serialport::SerialPort>, cmd: &'a str) -> Result<String, ()> {
    match port.write(cmd.as_bytes()) {
        Ok(_) => {
            let mut buffer: Vec<u8> = vec![0; 100];
            let mut res: Vec<u8> = vec![];
            for _ in 0..10 {
                std::thread::sleep(Duration::from_millis(50));
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
        match at_command(&mut port, "AT+GSN\r") {
            Ok(s) => {
                println!("{}", s);
                let mut lines = s.lines();
                lines.next();
                match lines.next() {
                    Some(s) if s == "ERROR" => Err(()),
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

    pub fn register(&mut self, server: &str, port: &str, lifetime: &str) -> Result<(), ()> {
        match at_command(
            &mut self.port,
            &format!(
                "AT+CM2MCLINEW=\"{}\",{},{},{}\r",
                server, port, self.imei, lifetime
            ),
        ) {
            Ok(s) => {
                println!("{}", s);
                let mut lines = s.lines();
                lines.next();
                match lines.next() {
                    Some(s) if s == "OK" => Ok(()),
                    Some(_) | None => Err(()),
                }
            }
            Err(_) => Err(()),
        }
    }

    pub fn send(&mut self, msg: &str) -> Result<(), ()> {
        match at_command(&mut self.port, &format!("AT+CM2MCLISEND=\"{}\"\r", msg)) {
            Ok(res) => {
                println!("{}", res);
                let mut lines = res.lines();
                lines.next();
                match lines.next() {
                    Some(s) if s == "OK" => Ok(()),
                    Some(_) | None => Err(()),
                }
            }
            Err(_) => Err(()),
        }
    }

    pub fn deregister(&mut self) -> Result<(), ()> {
        match at_command(&mut self.port, &format!("AT+CM2MCLIDEL\r")) {
            Ok(res) => {
                println!("{}", res);
                let mut lines = res.lines();
                lines.next();
                match lines.next() {
                    Some(s) if s == "OK" => Ok(()),
                    Some(_) | None => Err(()),
                }
            }
            Err(_) => Err(()),
        }
    }

    pub fn power_off(&mut self) -> Result<(), ()> {
        match at_command(&mut self.port, &format!("AT+CPOWD=1\r")) {
            Ok(res) => {
                println!("{}", res);
                let mut lines = res.lines();
                lines.next();
                match lines.next() {
                    Some(s) if s == "OK" => Ok(()),
                    Some(_) | None => Err(()),
                }
            }
            Err(_) => Err(()),
        }
    }
}
