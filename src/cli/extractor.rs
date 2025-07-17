use std::{collections::HashSet, net::IpAddr};
use std::error::Error;

use ipnetwork::IpNetwork;


/**
 * String format: 56,56,43,88-9000
 */
pub fn parse_port_input(to_parse: &str) -> Result<HashSet<u16>, Box<dyn Error + Send + Sync>> {
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
    Ok(set)
}

pub fn parse_ip_addr_input(to_parse: &str) -> Result<Vec<IpAddr>, Box<dyn Error + Send + Sync>> {

    if let Ok(net) = to_parse.parse::<IpNetwork>() {
        Ok(net.iter().collect())
    } else if let Ok(ip) = to_parse.parse::<IpAddr>() {
        Ok(vec![ip])
    } else {
        Err("Parse error, invalid IP".into())
    }
}


