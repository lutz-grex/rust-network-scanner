use std::{collections::HashSet, net::IpAddr};

use anyhow::anyhow;
use ipnetwork::IpNetwork;

/// Parses a string of ports and returns a HashSet of u16.
/// The string can contain individual ports or ranges (e.g., "80,443,8000-9000").
/// # Errors
///Returns an error if the input string is malformed or contains invalid port numbers.
pub fn parse_port_input(to_parse: &str) -> Result<HashSet<u16>, anyhow::Error> {
    let mut results = Vec::new();

    for part in to_parse.split(',') {
        let part = part.trim();
        if part.contains('-') {
            let bounds: Vec<_> = part.split('-').collect();
            if bounds.len() == 2 {
                if let (Ok(start), Ok(end)) = (
                    bounds[0].trim().parse::<u16>(),
                    bounds[1].trim().parse::<u16>(),
                ) {
                    results.extend(start..=end);
                }
            }
        } else {
            if let Ok(port) = part.parse::<u16>() {
                results.push(port);
            }
        }
    }

    let set: HashSet<u16> = results.into_iter().collect();
    Ok(set)
}

/// Parses a string of IP addresses or networks and returns a vector of `IpAddr`.
/// The string can contain individual IPs or CIDR notation (e.g., "192.
pub fn parse_ip_addr_input(to_parse: &str) -> Result<Vec<IpAddr>, anyhow::Error> {
    if let Ok(net) = to_parse.parse::<IpNetwork>() {
        Ok(net.iter().collect())
    } else if let Ok(ip) = to_parse.parse::<IpAddr>() {
        Ok(vec![ip])
    } else {
        Err(anyhow!("Parse error, invalid IP"))
    }
}
