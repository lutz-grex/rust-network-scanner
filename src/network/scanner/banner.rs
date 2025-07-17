use std::error::Error;
use std::io::{Read};
use std::time::{Duration};
use flate2::bufread::GzDecoder;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time;
use tokio::time::{timeout};

use crate::models::connection::WebService;


#[derive(Debug, Clone)]
pub struct BannerScanResult {
    pub banner: String,
    pub server: String,
    pub web_service: WebService,
}

// impl Default for BannerScanResult {
//     fn default() -> Self {
//         Self {
//             banner: String::new(),
//             server: String::new()
//         }
//     }
// }



pub async fn scan_banner(address: &str, timeout_duration: &Duration) -> Result<BannerScanResult, Box<dyn Error + Send + Sync>> {

    let mut stream = time::timeout(*timeout_duration, TcpStream::connect(address)).await??;

    if let Some(banner_result) = read_initial_banner(&mut stream).await? {
        return Ok(banner_result);
    }

    let request = format!("HEAD / HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n", address);
    let _ = stream.write_all(request.as_bytes()).await?;

    let request_content =  read_request_buffer_from_tcp_stream(&mut stream).await?;
    
    if let Some((header, _body)) = split_request_content(&request_content) {
        return Ok(BannerScanResult{
            banner: header.to_string(),
            server: get_banner_server(&header).unwrap_or_default(),
            web_service: if header.starts_with("HTTP/") {WebService::HTTP} else {WebService::NONE}
        });
    } else {
        Err("No header in response".into())
    }
}


/**
 * Initial read, without request   
 * Services like SSH, FTP, SMTP -> Without HTTP-Banner
 */
pub async fn read_initial_banner(stream: &mut TcpStream) -> Result<Option<BannerScanResult>, Box<dyn Error + Send + Sync>> {

    // It is not safe to assume that the server will send an end-of-file (EOF) —> therefore not read with read_request_buffer_from_tcp_stream
    let mut buffer = [0u8; 1024];
    
    let n = match timeout(Duration::from_millis(200), stream.read(&mut buffer)).await {
        Ok(Ok(n)) if n > 0 => n,
        _ => return Ok(None),
    };

    let response = String::from_utf8_lossy(&buffer[..n]).to_string();
     
    let response_upper = response.to_uppercase();


    let web_service = match true {
        _ if response.starts_with("SSH-") => WebService::SSH,
        _ if response.starts_with("220") && response_upper.contains("FTP") => WebService::FTP,
        _ if response.starts_with("220") && response_upper.contains("SMTP") => WebService::SMTP,
        _ if response.starts_with("+OK") => WebService::POP3,
        _ if response.starts_with("* OK") => WebService::IMAP,
        _ if response.starts_with("-ERR") || response.starts_with("+PONG") || response.starts_with("*") => WebService::REDIS,
        _ => return Ok(None),
    };

    Ok(Some(BannerScanResult {
        banner: response,
        server: String::new(),
        web_service: web_service
    }))
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
        .lines() // e.q. split("\r\n")
        .find(|line| line.to_uppercase().starts_with("SERVER:"))
        .map(|line| line.trim_start_matches("Server:").trim().to_string())
}


fn decompress_gzip(compressed_data: &[u8]) -> Result<String, Box<dyn Error>> {
    let mut decoder = GzDecoder::new(compressed_data);
    let mut decompressed = String::new();
    decoder.read_to_string(&mut decompressed)?;
    Ok(decompressed)
}