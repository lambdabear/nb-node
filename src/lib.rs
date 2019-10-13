use std::io;
use std::net::Ipv4Addr;
use std::str;
use std::str::FromStr;
use std::time::Duration;

// use serialport::prelude::*;

fn at_command<'a>(port: &mut Box<dyn serialport::SerialPort>, cmd: &'a str) -> Result<String, ()> {
    match port.write(cmd.as_bytes()) {
        Ok(_) => {
            let mut buffer: Vec<u8> = vec![0; 100];
            let mut res: Vec<u8> = vec![];
            for _ in 0..50 {
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
                lines.next();
                lines.next();
                match lines.next() {
                    Some(s) if s == "OK" => Ok(()),
                    Some(_) | None => Err(()),
                }
            }
            Err(_) => Err(()),
        }
    }

    pub fn enable_release_assistance(&mut self) -> Result<(), ()> {
        match at_command(&mut self.port, &format!("AT+CNBIOTRAI=1\r")) {
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

    pub fn signal_quality(&mut self) -> Result<String, ()> {
        match at_command(&mut self.port, &format!("AT+CSQ\r")) {
            Ok(res) => {
                // println!("{}", res);
                let mut lines = res.lines();
                lines.next();
                match lines.next() {
                    Some(s) => Ok(s.to_string()),
                    None => Err(()),
                }
            }
            Err(_) => Err(()),
        }
    }

    pub fn disable_psm(&mut self) -> Result<(), ()> {
        match at_command(&mut self.port, &format!("AT+CPSMS=0\r")) {
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

    pub fn rssi(&mut self) -> Result<u8, ()> {
        match at_command(&mut self.port, "AT+CSQ\r") {
            Ok(res) => {
                let mut lines = res.lines();
                lines.next();
                match lines.next() {
                    Some(s) => {
                        if s.len() > 9 {
                            let rssi = s[6..8].parse::<u8>();
                            match rssi {
                                Ok(r) => Ok(r),
                                Err(_) => Err(()),
                            }
                        } else {
                            Err(())
                        }
                    }
                    None => Err(()),
                }
            }
            Err(_) => Err(()),
        }
    }

    pub fn battery(&mut self) -> Result<u16, ()> {
        match at_command(&mut self.port, "AT+CBC\r") {
            Ok(res) => {
                let mut lines = res.lines();
                lines.next();
                match lines.next() {
                    Some(s) => {
                        if let Some((i, _)) = s.match_indices(",").next() {
                            let bat = s[i + 1..].parse::<u16>();
                            match bat {
                                Ok(b) => Ok(b),
                                Err(_) => Err(()),
                            }
                        } else {
                            Err(())
                        }
                    }
                    None => Err(()),
                }
            }
            Err(_) => Err(()),
        }
    }

    // check if sim card is ready
    pub fn sim_ready(&mut self) -> bool {
        match at_command(&mut self.port, "AT+CPIN?\r") {
            Ok(res) => {
                let mut lines = res.lines();
                match lines.nth(1) {
                    Some(s) => s == "+CPIN: READY",
                    None => false,
                }
            }
            Err(_) => false,
        }
    }

    // check if PDN is activated
    pub fn pdn_active(&mut self) -> bool {
        match at_command(&mut self.port, "AT+CGACT?\r") {
            Ok(res) => {
                let mut lines = res.lines();
                match lines.nth(1) {
                    Some(s) => match s.match_indices(",").nth(0) {
                        Some((i, _)) => {
                            if s.len() > i + 1 {
                                match s[i + 1..].parse::<u8>() {
                                    Ok(n) if n == 1 => true,
                                    _ => false,
                                }
                            } else {
                                false
                            }
                        }
                        None => false,
                    },
                    None => false,
                }
            }
            Err(_) => false,
        }
    }

    pub fn operator(&mut self) -> Result<String, ()> {
        match at_command(&mut self.port, "AT+COPS?\r") {
            Ok(res) => {
                let mut lines = res.lines();
                match lines.nth(1) {
                    Some(s) => {
                        println!("{}", s);
                        let mut matches = s.match_indices("\"");
                        match (matches.next(), matches.next()) {
                            (Some((i, _)), Some((j, _))) if j - i == 6 => {
                                Ok(String::from(&s[i + 1..j]))
                            }
                            _ => Err(()),
                        }
                    }
                    None => Err(()),
                }
            }
            Err(_) => Err(()),
        }
    }

    pub fn apn_ip_addr(&mut self) -> Result<(String, Ipv4Addr, Ipv4Addr), ()> {
        match at_command(&mut self.port, "AT+CGCONTRDP\r") {
            Ok(res) => {
                let mut lines = res.lines();
                match lines.nth(1) {
                    Some(s) => {
                        let mut quotes_matches = s.match_indices("\"");
                        let mut dot_matches = s.match_indices(".");
                        match (
                            quotes_matches.next(),
                            quotes_matches.next(),
                            quotes_matches.next(),
                            dot_matches.nth(3),
                            quotes_matches.next(),
                        ) {
                            (
                                Some((i, _)),
                                Some((j, _)),
                                Some((k, _)),
                                Some((l, _)),
                                Some((m, _)),
                            ) if j - i < 65 => {
                                let apn = String::from(&s[i + 1..j]);
                                match (
                                    Ipv4Addr::from_str(&s[k + 1..l]),
                                    Ipv4Addr::from_str(&s[l + 1..m]),
                                ) {
                                    (Ok(ip), Ok(mask)) => Ok((apn, ip, mask)),
                                    _ => Err(()),
                                }
                            }
                            _ => Err(()),
                        }
                    }
                    None => Err(()),
                }
            }
            Err(_) => Err(()),
        }
    }
}
