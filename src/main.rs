use std::net::{TcpStream};
use std::time::Duration;
use std::net::ToSocketAddrs;
use clap::Parser;

fn main() -> std::io::Result<()>{
    let args = Cli::parse();
    let d = Duration::new(args.timeout as u64, 0);
    let hostname = args.host;

    println!("Scanning {}", hostname);

    let ports = args.min_port..(args.max_port+1);

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

    Ok(())
}

#[derive(Parser, Debug)]
struct Cli {
    #[arg(long)]
    host: String,

    #[arg(short, long, default_value_t = 1)]
    timeout: u8,

    #[arg(long, default_value_t = 1)]
    min_port: u16,

    #[arg(long, default_value_t = 1)]
    max_port: u16
}
