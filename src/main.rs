


mod  cli;
mod services;
mod network;
mod routes;
mod thread_executor;
mod models;

use std::path::Path;
use anyhow::Result;


use clap::Parser;


#[macro_use]
extern crate rocket;

use crate::cli::extractor::parse_ip_addr_input;
use crate::cli::input::{Cli, Commands};
use crate::services::file::write_file;
use crate::thread_executor::thread_fetch_connection_details;

// #[launch]
// fn rocket() -> _ {
//     rocket::build()
//         .mount("/api", routes::scan::routes())
//         .mount("/api", routes::health::routes())
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Scan { target, ports, timeout, concurrency, banner, output } => {
            let ips = parse_ip_addr_input(&target);

            let connections = thread_fetch_connection_details(&ips, ports, *timeout, *concurrency, *banner).await?;

            let pretty_json = serde_json::to_string_pretty(&connections)
                .map_err(|e| anyhow::anyhow!("JSON Serialization failed: {}", e))?;

            if let Some(path) = output {
                write_file(Path::new(path), pretty_json)?;
            } else {
                println!("{}", pretty_json);
            }
        }
    }

    Ok(())

}
