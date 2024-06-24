import ipaddress
import argparse
import sys

def subnet_calculator(network: str):
    try:
        # Create the network object
        network = ipaddress.IPv4Network(network, strict=False)
        
        # Network information
        ip_info = {
            "IP Address": str(network.network_address),
            "Subnet Mask": str(network.netmask),
            "Network Address": str(network.network_address),
            "Broadcast Address": str(network.broadcast_address),
            "Number of Usable Hosts": network.num_addresses - 2,
            "First Usable IP": str(list(network.hosts())[0]),
            "Last Usable IP": str(list(network.hosts())[-1])
        }

        return ip_info
    
    except ValueError as e:
        return {"Error": str(e)}

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Subnet Calculator')
    parser.add_argument('network', type=str, nargs='?', help='Network in CIDR notation (e.g., 192.168.1.0/24)')
    
    args = parser.parse_args()
    
    if not args.network:
        print("Error: No network provided.")
        print("Usage: python subnet_calculator.py <network>")
        print("Example: python subnet_calculator.py 192.168.1.0/24")
        sys.exit(1)
    
    result = subnet_calculator(args.network)
    
    for key, value in result.items():
        print(f"{key}: {value}")
