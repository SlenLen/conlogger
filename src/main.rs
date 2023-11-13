// This is by no means supposed to be a professional program or anything similar.
// This is a very botchy program but it works I guess.


use ping_fox::{PingFoxConfig, PingReceive, PingReceiveData, PingSentToken, SocketType};
use std::net::Ipv4Addr;
use std::time::Duration;
use chrono::{DateTime, Local};
use std::fs::OpenOptions;
use std::io::prelude::*;
use clap::Parser;

/// simple logger that tells you at what times your device couldn't connect to the internet
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Write to stdout in addition to the log file
    #[arg(short, long)]
    verbose: bool,

    /// Set the timout after which a ping and the connection are deemed "lost"
    #[arg(short, long)]
    #[clap(default_value_t = 60)]
    timeout: u64,
}

fn main() {   
    let args = Args::parse();

    let mut connected: bool;
    let mut last_connected: bool = true;
    let mut last_change: DateTime<Local> = Local::now();

    let _now: DateTime<Local> = Local::now();

    let filename: String = _now.format("%v.log").to_string();
    let mut exe_dir = std::env::current_exe().expect("Couldn't get executable directory.");
    exe_dir.pop();
    exe_dir.push(filename);
    let filename = exe_dir.to_str().expect("Couldn't convert executable directory to string.");
    
    loop {
        let config = PingFoxConfig {
            socket_type: SocketType::DGRAM,
            timeout: Duration::from_millis(args.timeout),
            channel_size: 1,
        };
    
        let (mut ping_sender, mut ping_receiver) = ping_fox::create(&config).unwrap();
    
        let token: PingSentToken = ping_sender.send_to("8.8.8.8".parse::<Ipv4Addr>().unwrap()).unwrap();
        let ping_response = ping_receiver.receive(token);
    
        match ping_response {
            Ok(PingReceive::Data(PingReceiveData {
                package_size: _,
                ip_addr: _,
                ttl: _,
                sequence_number: _,
                ping_duration: _,
            })) => {
                connected = true;
            }
            Ok(PingReceive::Timeout) => {
                todo!("Should never be called. If you see this, wtf???");
            }
            Err(_) => {
                connected = false;
            },
        };

        if connected != last_connected {
            let _now: DateTime<Local> = Local::now();
            let output: String = format!("from {} to {} ({}ms): {} connection",
                                        last_change.format("%T%.3f"),
                                        _now.format("%T%.3f"),
                                        (_now - last_change).num_milliseconds(),
                                        if connected {"successful"} else {"lost"});

            if args.verbose {
                println!("{}", output);
            }

            last_change = Local::now();
            last_connected = connected;

            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(filename)
                .expect("Couldn't create or access log file.");

            match writeln!(file, "{}", output) {
                Ok(_v) => (),
                Err(e) => {println!("Couldn't write file!\n{e}")},
            };
        }
    }
}
    
