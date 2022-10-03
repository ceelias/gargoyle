use std::fs;

fn get_ephermeral_port_range() -> (i32, i32) {
    let contents = fs::read_to_string("/proc/sys/net/ipv4/ip_local_port_range")
        .expect("Unable to read ip local port range file");

    let port_range: Vec<&str> = contents.split_whitespace().collect();
    let low: i32 = port_range[0].parse().unwrap();
    let high: i32 = port_range[1].parse().unwrap();

    (low, high)
}

fn main() {
    println!("Getting ephermeral port range");
    let (ephermeral_low, ephermeral_high) = get_ephermeral_port_range();
    println!(
        "ephemeral port range: {:?}-{:?}",
        ephermeral_low, ephermeral_high
    );
}
