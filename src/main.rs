use std::net::{TcpStream};
use std::process::exit;
use std::time::Duration;
use std::net::ToSocketAddrs;

fn main() -> std::io::Result<()>{
    let d = Duration::new(1, 0);
    let hostname = match std::env::args().nth(1) {
        Some(address) => address,
        None => {
            println!("no address provided");
            exit(1)
        }
    };

    println!("Scanning {}", hostname);

    let min_port = std::env::args().nth(2).
        and_then(|t| {Some(t.parse::<u32>().expect("invalid min port"))}).
        unwrap_or(1);

    let max_port = std::env::args().nth(3).
        and_then(|t| {Some(t.parse::<u32>().expect("invalid max port"))}).
        unwrap_or(1);

    let ports = min_port..(max_port+1);

    for i in ports {
        let address = hostname.clone() + ":" + &i.to_string();
        // TODO: Resolve domain names outside of the port loop
        let mut addresses = address.to_socket_addrs().unwrap();
        let sockaddr = match addresses.len() {
            0 => panic!("Could not resolve address {}", hostname),
            1 => addresses.nth(0).expect("kaboom 1"),
            _ => {
                println!("Note: Multiple IPs found for address {}", hostname);
                addresses.nth(0).expect("kaboom 2")
            }
        };
        match TcpStream::connect_timeout(&sockaddr, d) {
            Err(err) => println!("🙅 {}: {}", i, err),
            Ok(_) => println!("🤗 {}", i)
        }
    }


    // Read address from command line

    // Range over ports 0..10000, or read min/max ports from CLI args

    // Try to open a TCP connection, only wait ~1 second


    Ok(())
}
