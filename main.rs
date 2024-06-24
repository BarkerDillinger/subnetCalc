use clap::Parser;
use std::net::Ipv4Addr;
use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Network in CIDR notation (e.g., 192.168.1.0/24)
    #[arg(short = 'n', long = "network")]
    network: Option<String>,
}

fn subnet_calculator(network: &str) -> Result<(), String> {
    let parts: Vec<&str> = network.split('/').collect();
    if parts.len() != 2 {
        return Err("Invalid CIDR notation".to_string());
    }

    let ip = Ipv4Addr::from_str(parts[0]).map_err(|_| "Invalid IP address".to_string())?;
    let prefix = parts[1].parse::<u8>().map_err(|_| "Invalid prefix length".to_string())?;
    if prefix > 32 {
        return Err("Prefix length must be between 0 and 32".to_string());
    }

    let subnet_mask = !((1u32 << (32 - prefix)) - 1);
    let network_address = u32::from(ip) & subnet_mask;
    let broadcast_address = network_address | !subnet_mask;
    let num_usable_hosts = if prefix == 32 { 1 } else { (1u32 << (32 - prefix)) - 2 };

    let first_usable_ip = if prefix == 32 {
        network_address
    } else {
        network_address + 1
    };
    let last_usable_ip = if prefix == 32 {
        network_address
    } else {
        broadcast_address - 1
    };

    println!("IP Address: {}", ip);
    println!("Subnet Mask: {}", Ipv4Addr::from(subnet_mask));
    println!("Network Address: {}", Ipv4Addr::from(network_address));
    println!("Broadcast Address: {}", Ipv4Addr::from(broadcast_address));
    println!("Number of Usable Hosts: {}", num_usable_hosts);
    println!("First Usable IP: {}", Ipv4Addr::from(first_usable_ip));
    println!("Last Usable IP: {}", Ipv4Addr::from(last_usable_ip));

    Ok(())
}

fn main() {
    let args = Args::parse();

    match args.network {
        Some(network) => {
            if let Err(err) = subnet_calculator(&network) {
                eprintln!("Error: {}", err);
                println!("Usage: subnet_calculator --network <IP_ADDRESS>/<CIDR> or -n <IP_ADDRESS>/<CIDR>");
                println!("Example: subnet_calculator --network 192.168.1.0/24 or -n 192.168.1.0/24");
            }
        }
        None => {
            println!("Please provide an IP Address with CIDR Notation for the Subnet Mask.");
            println!("Usage: subnet_calculator --network <IP_ADDRESS>/<CIDR> or -n <IP_ADDRESS>/<CIDR>");
            println!("Example: subnet_calculator --network 192.168.1.0/24 or -n 192.168.1.0/24");
        }
    }
}