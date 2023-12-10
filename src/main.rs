// This is by no means supposed to be a professional program or anything similar.
// This is a very botchy program but it works I guess.
#![feature(io_error_more)]

use connection::{get_connection_status, ADDRS};
use chrono::{DateTime, Local};
use std::fs::OpenOptions;
use std::io::prelude::*;
use clap::Parser;

use crate::connection::ConnectionStatus;

mod connection;

/// A simple connection status logger
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Write to stdout in addition to the log file
    #[arg(short, long)]
    verbose: bool,

    /// Set the timout after which a ping and the connection are deemed "lost"
    #[arg(short, long)]
    #[clap(default_value_t = 120)]
    timeout: u64,
}

fn main() {   
    let args = Args::parse();

    let mut status: ConnectionStatus;
    let mut last_status: ConnectionStatus = ConnectionStatus::ConnectionUp;
    let mut last_change: DateTime<Local> = Local::now();

    let _now: DateTime<Local> = Local::now();

    let filename: String = _now.format("%v.log").to_string();
    let mut exe_dir = std::env::current_exe().expect("Couldn't get executable directory.");
    exe_dir.pop();
    exe_dir.push(filename);
    let filename = exe_dir.to_str().expect("Couldn't convert executable directory to string.");
    
    loop {
        let test = get_connection_status(&ADDRS, std::time::Duration::from_millis(args.timeout));
        status = test.unwrap();


        if status != last_status {
            let _now: DateTime<Local> = Local::now();
            let output: String = format!("from {} to {} ({}ms): {}",
                last_change.format("%T%.3f"),
                _now.format("%T%.3f"),
                (_now - last_change).num_milliseconds(),
                match last_status {
                    ConnectionStatus::ConnectionUp => "connection up",
                    ConnectionStatus::ConnectionDown => "connection down",
                    ConnectionStatus::NoReachableNetwork => "no reachable network"
                });

            if args.verbose {
                println!("{}", output);
            }

            last_change = Local::now();
            last_status = status;

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
    
