use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::net::Ipv4Addr;

#[derive(Debug)]
struct Route {
    network: Ipv4Addr,
    mask: Ipv4Addr,
    action: String,
}

fn parse_ipv4(s: &str) -> Ipv4Addr {
    s.parse().expect("Invalid IP address format")
}

fn ip_matches(ip: Ipv4Addr, route: &Route) -> bool {
    let ip_u32 = u32::from(ip);
    let net_u32 = u32::from(route.network);
    let mask_u32 = u32::from(route.mask);
    (ip_u32 & mask_u32) == (net_u32 & mask_u32)
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

        println!("{} {}", ip, action);
    }
}