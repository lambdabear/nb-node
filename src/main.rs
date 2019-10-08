use clap::{App, Arg, SubCommand};
use nb_node::Node;
use serialport::prelude::*;

fn main() {
    let matches = App::new("NB")
        .version("0.1")
        .author("CrazyBear")
        .about("NB-iot module cli")
        .arg(
            Arg::with_name("server")
                .short("s")
                .long("server")
                .default_value("221.229.214.202")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .default_value("5683")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("timeout")
                .short("t")
                .long("timeout")
                .default_value("86400")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("serial-dev")
                .short("d")
                .long("dev")
                .default_value("/dev/ttyUSB0")
                .takes_value(true),
        )
        .subcommands(vec![
            SubCommand::with_name("register").about("register NB node in CT-iot cloud"),
            SubCommand::with_name("deregister").about("deregister NB node in CT-iot cloud"),
            SubCommand::with_name("enable-release-assistance").about("enable release assistance"),
            SubCommand::with_name("power-off").about("power off NB node"),
            SubCommand::with_name("signal-quality").about("signal quality report"),
            SubCommand::with_name("disable-psm").about("disable us of PSM"),
            SubCommand::with_name("send")
                .about("send data to CT-iot cloud")
                .arg(Arg::with_name("data").takes_value(true).required(true)),
        ])
        .get_matches();

    let server = matches.value_of("server").expect("get server error");
    let s_port = matches.value_of("port").expect("get port error");
    let timeout = matches.value_of("timeout").expect("get timeout error");
    let serial_dev = matches
        .value_of("serial-dev")
        .expect("get serial dev error");

    let mut settings: SerialPortSettings = Default::default();
    settings.baud_rate = 115200;

    match serialport::open_with_settings(serial_dev, &settings) {
        Ok(port) => match Node::new(port) {
            Ok(mut node) => {
                println!("create NB module succeed! imei: {}", node.get_imei());
                if let Some(_) = matches.subcommand_matches("register") {
                    match node.register(server, &s_port, &timeout) {
                        Ok(_) => println!("register succeed"),
                        Err(_) => println!("register failed"),
                    };
                }

                if let Some(_) = matches.subcommand_matches("deregister") {
                    match node.deregister() {
                        Ok(_) => println!("deregister succeed"),
                        Err(_) => println!("deregister failed"),
                    };
                }

                if let Some(_) = matches.subcommand_matches("signal-quality") {
                    for _ in 0..10 {
                        match node.signal_quality() {
                            Ok(s) => println!("{}", s),
                            Err(_) => println!("test signal quality failed"),
                        };
                    }
                }

                if let Some(_) = matches.subcommand_matches("power-off") {
                    match node.power_off() {
                        Ok(_) => println!("power off succeed"),
                        Err(_) => println!("power off failed"),
                    };
                }

                if let Some(_) = matches.subcommand_matches("disable-psm") {
                    match node.disable_psm() {
                        Ok(_) => println!("disable PSM succeed"),
                        Err(_) => println!("disable PSM failed"),
                    };
                }

                if let Some(_) = matches.subcommand_matches("enable-release-assistance") {
                    match node.enable_release_assistance() {
                        Ok(_) => println!("enable release assistance succeed"),
                        Err(_) => println!("enable release assistance failed"),
                    };
                }

                if let Some(matches) = matches.subcommand_matches("send") {
                    let data = matches.value_of("data").expect("need valid data");
                    match node.send(data) {
                        Ok(_) => println!("sending data succeed"),
                        Err(_) => println!("sending data failed"),
                    };
                }
            }
            Err(_) => println!("creat nb node error"),
        },
        Err(_) => println!("open serial port error"),
    }
}
