use std::error::Error;
use std::io::{Read, Write};
use std::time::{Duration, Instant};
use flate2::bufread::GzDecoder;
use rocket::outcome::IntoOutcome;
use serde::ser;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time;
use std::net::{IpAddr, TcpStream as StdTcpStream};
use regex::Regex;



pub struct BannerScanResult {
    pub banner: String,
    pub server: String
}

pub async fn scan_banner(address: &str, timeout_duration: &Duration) -> Result<BannerScanResult, Box<dyn Error + Send + Sync>> {

    let mut stream = time::timeout(*timeout_duration, TcpStream::connect(address)).await??;
        // Use HEAD request to avoid getting the body content
    let request = format!("HEAD / HTTP/1.0\r\nHost: {}\r\n\r\n", address);
    let _ = stream.write_all(request.as_bytes()).await?;

    let request_content =  read_request_buffer_from_tcp_stream(&mut stream).await?;
    
    if let Some(content) = split_request_content(&request_content) {
        let header = content.0.to_string();
        
        return Ok(BannerScanResult{
            banner: header.clone(),
            server: get_banner_server(&header).unwrap_or_default()
        });
    } else {
        Err("No header in response".into())
    }
}



async fn read_request_buffer_from_tcp_stream(stream: &mut TcpStream) -> Result<String, Box<dyn Error + Send + Sync>> {

    let mut response = Vec::new(); 
    let mut buffer = [0; 1024];

    loop {
        match stream.read(&mut buffer).await {
            Ok(0) => break,
            Ok(size) => response.extend_from_slice(&buffer[..size]),
            Err(e) => return Err(Box::new(e)),
        };
    };

    Ok(String::from_utf8_lossy(&response).into_owned())
}

// headers, body
fn split_request_content(request_content: &str) -> Option<(&str, &str)> {
    request_content.split_once("\r\n\r\n")
}

fn get_banner_server(banner: &str) -> Option<String> {
    banner
        .split("\r\n")
        .find(|line| line.to_uppercase().starts_with("SERVER:"))
        .map(|line| line.trim_start_matches("Server:").trim().to_string())
}


fn decompress_gzip(compressed_data: &[u8]) -> Result<String, Box<dyn Error>> {
    let mut decoder = GzDecoder::new(compressed_data);
    let mut decompressed = String::new();
    decoder.read_to_string(&mut decompressed)?;
    Ok(decompressed)
}