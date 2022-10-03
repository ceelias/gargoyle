use default_net;
use local_ip_address::list_afinet_netifas;
use std::fs;

fn get_ephermeral_port_range() -> (i32, i32) {
    let contents = fs::read_to_string("/proc/sys/net/ipv4/ip_local_port_range")
        .expect("Unable to read ip local port range file");

    let port_range: Vec<&str> = contents.split_whitespace().collect();
    let low: i32 = port_range[0].parse().unwrap();
    let high: i32 = port_range[1].parse().unwrap();

    (low, high)
}

fn get_listening_ports_to_ignore() -> Vec<i32> {
    let contents = fs::read_to_string("/proc/net/tcp").expect("Unable to read tcp file");
    let mut ignore_ports: Vec<i32> = vec![];

    let lines = contents.lines();

    for line in lines {
        if line.contains(":") {
            let local_ip_ports: Vec<&str> = line.split_whitespace().collect();
            let ports: Vec<&str> = local_ip_ports[1].split(':').collect();
            let port = i32::from_str_radix(ports[1], 16).unwrap(); // Unwrap gets val from Result type

            ignore_ports.push(port); // Could potentially add a check to see if it falls in the eph range
        }
    }

    ignore_ports
}

fn get_local_ips() -> Vec<String> {
    let mut local_ips: Vec<String> = vec![];

    let network_interfaces = list_afinet_netifas().unwrap();

    for (_, ip) in network_interfaces.iter() {
        local_ips.push(ip.to_string());
    }

    local_ips
}

fn get_default_gateway() -> String {
    let default_gateway = default_net::get_default_gateway().unwrap();
    default_gateway.ip_addr.to_string()
}

fn main() {
    println!("Getting ephermeral port range");
    let (ephermeral_low, ephermeral_high) = get_ephermeral_port_range();
    println!(
        "ephemeral port range: {:?}-{:?}",
        ephermeral_low, ephermeral_high
    );
    let ignore_listeining_ports = get_listening_ports_to_ignore();

    println!("listening ports to ignore: {:?}", ignore_listeining_ports);

    let local_ips = get_local_ips();
    println!("local ips: {:?}", local_ips);

    let default_gateway_ip = get_default_gateway();
    println!("gateway ip: {:?}", default_gateway_ip);
}
