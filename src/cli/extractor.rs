use std::{collections::HashSet, net::IpAddr};

use ipnetwork::IpNetwork;


/**
 * String format: 56,56,43,88-9000
 */
pub fn parse_port_input(to_parse: &str) -> HashSet<u16> {
    let mut results = Vec::new();

    for part in to_parse.split(',') {
        let part = part.trim();
        if part.contains('-') {
            let bounds: Vec<_> = part.split('-').collect();
            if bounds.len() == 2 {
                if let (Ok(start), Ok(end)) = (bounds[0].trim().parse::<u16>(), bounds[1].trim().parse::<u16>()) {
                    results.extend(start..=end);
                }
            }
        } else {
            if let Ok(port) = part.parse::<u16>() {
                results.push(port);
            }
        }
    };

    let set: HashSet<u16> = results.into_iter().collect();
    set
}

pub fn parse_ip_addr_input(to_parse: &str) -> Vec<IpAddr> {

    if let Ok(net) = to_parse.parse::<IpNetwork>() {
        net.iter().collect()
    } else if let Ok(ip) = to_parse.parse::<IpAddr>() {
        vec![ip]
    } else {
        vec![]
    }
}


