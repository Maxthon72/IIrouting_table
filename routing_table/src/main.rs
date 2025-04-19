use std::env;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Route {
    network: u32,
    mask: u32,
    action: String,
}

fn parse_ipv4(s: &str) -> u32 {  
    let ip: Vec<&str>=s.split(".").collect();
    if ip.len() != 4
    {
        panic!("Invalid IPv4 address");
    }
    let mut ip_8: Vec<u32> = Vec::new();
    for i in ip.iter()
    {   
        ip_8.push(i.parse::<u32>().expect("Invalid octet"));
    }
    let ip_32: u32 = (ip_8[0] << 24) + (ip_8[1] << 16) + (ip_8[2] << 8) + (ip_8[3]);
    return ip_32;
}

fn ip_matches(ip: u32, route: &Route) -> bool {
    (ip & route.mask) == (route.network & route.mask)
}

fn read_routes(filename: &str) -> Vec<Route> {
    let file = File::open(filename).expect("Unable to open file");
    let reader = io::BufReader::new(file);

    reader.lines()
        .map(|line| {
            let line = line.expect("Unable to read line");
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() != 3 {
                panic!("Invalid route line format");
            }
            Route {
                network: parse_ipv4(parts[0]),
                mask: parse_ipv4(parts[1]),
                action: parts[2].to_string(),
            }
        })
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <routes_file> <ip1> <ip2> ...", args[0]);
        return;
    }

    let routes = read_routes(&args[1]);

    for ip_str in &args[2..] {
        let ip = parse_ipv4(ip_str);
        let mut action = "DROP".to_string();

        for route in &routes {
            if ip_matches(ip, route) {
                action = route.action.clone();
                break;
            }
        }

        println!("{} {}", ip_str, action);
    }
}