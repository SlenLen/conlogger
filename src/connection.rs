use std::net::{TcpStream, SocketAddr};
use std::io::{Error, ErrorKind};

pub const ADDRS: [&str; 2] = [
    "8.8.8.8:853", // Google DNS 
    "1.1.1.1:853", // Cloudflare DNS
];

#[derive(Debug)]
#[derive(PartialEq)]
pub enum ConnectionStatus {
    ConnectionUp,
    ConnectionDown,
    NoReachableNetwork
}

fn check_connection(addr: &str, timeout: std::time::Duration) -> Result<ConnectionStatus, Error> {
    let socket_addr = addr.parse::<SocketAddr>().unwrap();

    if let Err(e) = TcpStream::connect_timeout(&socket_addr, timeout) {
        match e.kind() {
            ErrorKind::TimedOut => { return Ok(ConnectionStatus::ConnectionDown); },
            ErrorKind::NetworkUnreachable => { return Ok(ConnectionStatus::NoReachableNetwork) },
            _ => { return Err(e); },
        }
    } else {
         return Ok(ConnectionStatus::ConnectionUp);
    }
}

pub fn get_connection_status(addresses: &[&str], timeout: std::time::Duration) -> Result<ConnectionStatus, Error> {
    for addr in addresses {
        match check_connection(addr, timeout)? {
            ConnectionStatus::ConnectionUp => return Ok(ConnectionStatus::ConnectionUp),
            ConnectionStatus::NoReachableNetwork => return Ok(ConnectionStatus::NoReachableNetwork),
            _ => (),
        }
    }
    return Ok(ConnectionStatus::ConnectionDown);
}