#[allow(unused_imports)]
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Write;
use std::io::Read;
use std::thread;


fn get_directory_arg() -> String{
    let args: Vec<String> = std::env::args().collect();
    let mut directory = String::new();
    
    for (i, arg) in args.iter().enumerate() {
        if arg == "--directory" {
            if let Some(dir)= args.get(i + 1) {
                directory = dir.clone();
                break;
            }
        }
    }
    directory
}

fn process_request(stream: &mut TcpStream) -> (String, String, Vec<String>, String) {
    let mut buffer = [0u8; 1024];
    let bytes_read = stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    
    let request_line = request.split_once("\r\n").unwrap().0;
    let line_path: Vec<&str> = request_line.split(" ").collect();
    let method = line_path[0].to_string();
    let url_path = line_path[1].to_string();
    
    let headers = request.split("\r\n")
        .filter(|l| !l.contains(":"))
        .map(|l| l.to_string())
        .collect::<Vec<String>>();
    
    let content = request.split_once("\r\n\r\n").map(|(_, c)| c.to_string()).unwrap_or_default();
    
    (method, url_path, headers, content)
}

fn handle_connection(mut stream: TcpStream) {
    // let response = "HTTP/1.1 200 OK\r\n\r\n";
    // stream.write_all(response.as_bytes()).unwrap();
    
    
    let directory = get_directory_arg();
    let (method, url_path, headers, content) = process_request(&mut stream);
    
    let response = match url_path {
        path if path == "/" => "HTTP/1.1 200 OK\r\n\r\n".to_string(),
        path if path.starts_with("/echo") => {
            let echo_content = path.replace("/echo/", "");
            format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", echo_content.len(), echo_content).to_string()
        },
        path if path.starts_with("/files/") && method == "GET" => {
            let file_path = path.replace("/files/", "");
            let full_path = std::path::Path::new(&directory).join(&file_path);
            
            if full_path.exists() {
                let content = std::fs::read_to_string(full_path).unwrap_or_default();
                format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}", content.len(), content).to_string()
            }else{
                "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
            }
        },
        path if path.starts_with("/files/") && method == "POST" => {
            let file_path = path.replace("/files/", "");
            let full_path = std::path::Path::new(&directory).join(&file_path);
            
            let _ = std::fs::write(full_path, content);
       
            "HTTP/1.1 201 Created\r\n\r\n".to_string()   
        },
        path if path.starts_with("/user-agent") => {
          let ua = headers.iter().find(|h| h.starts_with("User-Agent:")).unwrap().replace("User-Agent: ", "");
          format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", ua.len(), ua).to_string()
        },
        _ => "HTTP/1.1 404 Not Found\r\n\r\n".to_string(),
    };
        
   stream.write_all(response.as_bytes()).unwrap();
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
