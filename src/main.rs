use std::io::Read;
use std::io::Write;
#[allow(unused_imports)]
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

use flate2::Compression;
use flate2::write::GzEncoder;

fn gzip_compress(content: &str) -> Vec<u8> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(content.as_bytes()).unwrap();
    encoder.finish().unwrap()
}

fn get_directory_arg() -> String {
    let args: Vec<String> = std::env::args().collect();
    let mut directory = String::new();

    for (i, arg) in args.iter().enumerate() {
        if arg == "--directory" {
            if let Some(dir) = args.get(i + 1) {
                directory = dir.clone();
                break;
            }
        }
    }
    directory
}

fn process_request(stream: &mut TcpStream) -> Option<(String, String, Vec<String>, String)> {
    let mut buffer = [0u8; 1024];
    let bytes_read = stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..bytes_read]);

    let request_line = request.split_once("\r\n").unwrap().0;
    let line_path: Vec<&str> = request_line.split(" ").collect();
    let method = line_path[0].to_string();
    let url_path = line_path[1].to_string();

    let headers = request
        .split("\r\n")
        .filter(|l| l.contains(":"))
        .map(|l| l.to_string())
        .collect::<Vec<String>>();

    let content = request
        .split_once("\r\n\r\n")
        .map(|(_, c)| c.to_string())
        .unwrap_or_default();

    Some((method, url_path, headers, content))
}

fn get_header(headers: &[String], name: &str) -> String {
    headers
        .iter()
        .find(|h| h.starts_with(name))
        .and_then(|h| h.split_once(":"))
        .map(|(_, value)| value.trim().to_string())
        .unwrap_or_default()
}

fn handle_connection(mut stream: TcpStream) {
    // let response = "HTTP/1.1 200 OK\r\n\r\n";
    // stream.write_all(response.as_bytes()).unwrap();

    let directory = get_directory_arg();

    loop {
        let (method, url_path, headers, content) = match process_request(&mut stream) {
            Some(data) => data,
            _ => break,
        };
        
        let conn_header = get_header(&headers, "Connection");

        let mut compressed_content = Vec::<u8>::new();

        let response = match url_path {
            path if path == "/" => "HTTP/1.1 200 OK\r\n\r\n".to_string(),
            path if path.starts_with("/echo") => {
                let echo_content = path.replace("/echo/", "");

                let mut response: String;
                let accept_encoding = get_header(&headers, "Accept-Encoding");
                if accept_encoding.contains("gzip") {
                    compressed_content = gzip_compress(&echo_content);
                    response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Encoding: {}\r\nContent-Length: {}\r\n\r\n",
                        "gzip".to_string(),
                        compressed_content.len()
                    );
                } else {
                    response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", echo_content.len(), echo_content).to_string();
                }
                
                if conn_header.to_lowercase() == "close" {
                    response = response.replace("\r\n\r\n", "\r\nConnection: close\r\n\r\n");
                }

                response.to_string()
            }
            path if path.starts_with("/files/") && method == "GET" => {
                let file_path = path.replace("/files/", "");
                let full_path = std::path::Path::new(&directory).join(&file_path);

                if full_path.exists() {
                    let content = std::fs::read_to_string(full_path).unwrap_or_default();
                    format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}", content.len(), content).to_string()
                } else {
                    "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
                }
            }
            path if path.starts_with("/files/") && method == "POST" => {
                let file_path = path.replace("/files/", "");
                let full_path = std::path::Path::new(&directory).join(&file_path);

                let _ = std::fs::write(full_path, content);
                String::from("HTTP/1.1 201 Created\r\n\r\n")
            }
            path if path.starts_with("/user-agent") => {
                let ua = get_header(&headers, "User-Agent");
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    ua.len(),
                    ua
                )
                .to_string()
            }
            _ => "HTTP/1.1 404 Not Found\r\n\r\n".to_string(),
        };

        stream.write_all(response.as_bytes()).unwrap();

        if compressed_content.len() > 0 {
            stream.write_all(&compressed_content).unwrap();
        }
        

        if conn_header.to_lowercase() == "close" {
            break;
        }
    }
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // TODO: Uncomment the code below to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").expect("Could not bind to port");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                thread::spawn(move || {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
