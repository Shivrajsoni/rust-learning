// We need to declare the new module so that Rust knows to look for `thread_pool.rs`
mod thread_pool;

use crate::thread_pool::ThreadPool;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

// --- Teaching Note ---
// The old, unimplemented ThreadPool, Worker, and Job structs that were here have been removed.
// They are now replaced by our complete implementation in `thread_pool.rs`.
// This is good practice for organizing code into modules.

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub query: HashMap<String, String>,
    pub content: String,
}

#[derive(Debug)]
pub struct Response {
    status_text: String,
    headers: Vec<(String, String)>,
    body: String,
}

impl Response {
    pub fn json(status: u16, body: &str, headers: Option<Vec<(String, String)>>) -> Self {
        let content_len = body.len();
        let predetermined_headers = vec![
            ("Content-Type".to_string(), "application/json".to_string()),
            ("Content-Length".to_string(), content_len.to_string()),
        ];

        let headers = headers.unwrap_or_else(|| vec![]);

        let status_text = match status {
            200 => "200 OK".to_string(),
            400 => "400 Bad Request".to_string(),
            500 => "500 Internal Server Error".to_string(),
            _ => format!("{} Unknown ", status),
        };

        Self {
            status_text,
            headers: [predetermined_headers, headers].concat(),
            body: body.to_string(),
        }
    }

    pub fn resolve(response: &Response) -> String {
        let mut response_str = format!(
            "HTTP/1.1 {}
",
            response.status_text
        );

        for (key, value) in &response.headers {
            response_str.push_str(&format!(
                "{}: {}
",
                key, value
            ));
        }

        response_str.push_str("\r\n");
        response_str.push_str(&response.body);

        response_str
    }
}

const MESSAGE_SIZE: usize = 1024;

impl Request {
    pub fn new(mut stream: &TcpStream) -> Result<Self, String> {
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
                    return Err(err.to_string());
                }
            }
        }

        let request_text = String::from_utf8(recieved).unwrap();
        let mut request_lines: Vec<&str> = request_text.split_inclusive('\n').collect();
        let mut header_map: HashMap<String, String> = HashMap::new();
        let mut query_params: HashMap<String, String> = HashMap::new();
        let request_line = request_lines[0];
        let mut parts = request_line.split_ascii_whitespace();
        let http_method = parts.next().unwrap().to_string();
        let full_path = parts.next().unwrap();
        let path_and_query: Vec<&str> = full_path.split('?').collect();
        let path = path_and_query[0].to_string();

        if path_and_query.len() > 1 {
            let query_string = path_and_query[1..].join("");
            let query_pairs: Vec<&str> = query_string.split("&").collect();
            for pairs in query_pairs {
                if let Some((key, value)) = pairs.split_once("=") {
                    query_params.insert(key.to_string(), value.to_string());
                }
            }
        }

        request_lines.remove(0);
        let blank_line_index = request_lines.iter().position(|&line| line == "\r\n").unwrap();
        let mut body_lines = &mut request_lines.split_off(blank_line_index);
        body_lines.remove(0);
        let body_content = body_lines.join("");

        for header_line in &request_lines {
            if header_line.trim().is_empty() {
                continue;
            }
            if let Some((key, value)) = header_line.split_once(": ") {
                header_map.insert(key.to_string(), value.trim().to_string());
            }
        }

        Ok(Self {
            method: http_method,
            path,
            headers: header_map,
            query: query_params,
            content: body_content,
        })
    }
}

fn main() {
    println!("Working on Http from scratch");

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server listening on port 7878 with a thread pool.");

    // --- Teaching Note ---
    // Here we create our new ThreadPool.
    // A size of 4 is a common default. In a real-world application, this might be
    // configured based on the number of CPU cores on the machine.
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection Established!");

                // --- Teaching Note ---
                // This is the core change. Instead of spawning an infinite number of threads,
                // we pass a closure to `pool.execute`. The pool will then hand this closure
                // to one of its available worker threads to be run.
                // The `move` keyword is used to transfer ownership of the `stream` variable
                // to the closure, which is necessary because the closure will be run on a
                // different thread.
                pool.execute(move || {
                    handle_connection(stream);
                });

                /*
                --- This is the old, commented-out logic ---
                // This is the "thread per request" model, which we have now replaced.
                // It is inefficient because it creates a new thread for every single connection,
                // which can overwhelm the operating system under heavy load.
                std::thread::spawn(move || {
                    handle_connection(stream);
                });
                */
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    println!("Shutting down main thread.");
}

fn handle_connection(mut stream: TcpStream) {
    let req = Request::new(&stream);
    let res = match req {
        Ok(req) => match req.path.as_str() {
            "/hello" => {
                let def_name = String::from("Shivraj");
                let name: &String = req.query.get("name").unwrap_or_else(|| &def_name);
                let payload = format!("{{\"message\": \"Hello, {}!\"}}", name);
                Response::json(200, &payload, None)
            }
            _ => {
                let payload = "{{\"message\": \"Invalid Path\"}}";
                Response::json(400, &payload, None)
            }
        },
        Err(e) => {
            let payload = format!("{{\"message\": \"Error: {}}}", e);
            Response::json(500, &payload, None)
        }
    };
    let response_str = Response::resolve(&res);
    match stream.write(response_str.as_bytes()) {
        Ok(_) => {}
        Err(_) => {
            println!("FAILED DISPATCHED RESPONSE");
        }
    }
}