use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub query: HashMap<String, String>,
    pub content: String,
}

const MESSAGE_SIZE: usize = 1024;

impl Request {
    pub fn new(mut stream: &TcpStream) -> Result<Self, String> {
        // processing code
        let mut recieved: Vec<u8> = vec![];
        let mut rx_bytes = [0u8; MESSAGE_SIZE];

        loop {
            let bytes_read = stream.read(&mut rx_bytes);
            match bytes_read {
                Ok(bytes) => {
                    recieved.extend_from_slice(&rx_bytes[..bytes]);
                    if bytes < MESSAGE_SIZE {
                        break;
                    }
                }
                Err(err) => {
                    println!("Error : {:#?}", err);
                }
            }

            let request_text = String::from_utf8(recieved).unwrap();

            let mut request_lines: Vec<&str> = request_text.split_inclusive('\n').collect();

            let mut header_map: HashMap<String, String> = HashMap::new();
            let mut query_params: HashMap<String, String> = HashMap::new();

            // GET /path?x=1 HTTP/1.1

            let request_line = request_lines[0];
            let mut parts = request_line.split_ascii_whitespace();

            let http_method = parts.next().unwrap();
            let full_path = parts.next().unwrap();

            // seperate path and query string
            let path_and_query: Vec<&str> = full_path.split('?').collect();

            let path = path_and_query[0];

            if path_and_query.len() > 1 {
                let query_string = path_and_query[1..].join("");
                let query_pairs: Vec<&str> = query_string.split("&").collect();
                for pairs in query_pairs {
                    if let Some((key, value)) = pairs.split_once("=") {
                        query_params.insert(key.to_string(), value.to_string());
                    }
                }
            }

            // we have manipulated the first line of request lines so we can remove it
            request_lines.remove(0);

            let blank_line_index = request_lines
                .iter()
                .position(|&line| line == "\r\n")
                .unwrap();
            let body_lines = &mut request_lines.split_off(blank_line_index);
            body_lines.remove(0); // remove the blank line itself  
            let body_content = body_lines.join("");

            for header_line in &request_lines {
                if header_line.trim().is_empty() {
                    continue;
                }
                if let Some((key, value)) = header_line.split_once(": ") {
                    header_map.insert(key.to_string(), value.trim().to_string());
                }
            }
        }
    }
}

fn main() {
    println!("Working on Http from scratch");

    // Bind a listener to a port.
    // Using a port like 7878 which is less likely to be in use.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server listening on port 7878");

    // Accept connections and process them.
    // thread per connections architecture
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection Established!");
                std::thread::spawn(move || {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    // Read the request line
    let request_line = buf_reader.lines().next();

    // Ensure request_line is not None and unwrap it
    if let Some(Ok(request)) = request_line {
        println!("Request: {}", request);

        // A simple response
        let status_line = "HTTP/1.1 200 OK";
        let contents = "<h1>Hello, from your Rust server!</h1>";
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
        println!("Response sent.");
    } else {
        eprintln!("Failed to read request line.");
    }
}

