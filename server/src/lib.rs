mod routes;
mod thread_pool;

use routes::*;
use std::{
    collections::HashMap,
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use thread_pool::ThreadPool;

struct HttpRequest {
    pub method: String,
    pub uri: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

struct HttpResponse {
    pub code: u32,
    pub message: String,
    pub body: String,
}

pub fn run() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    let pool = ThreadPool::new(None);
    for stream in listener.incoming() {
        pool.execute(|| {
            let stream = match stream {
                Ok(value) => value,
                Err(_) => return,
            };
            handle_connection(stream);
        })
    }
}

fn error(code: u32, body: &str) -> HttpResponse {
    match code {
        400 => HttpResponse {
            code: 400,
            message: "BAD REQUEST".to_string(),
            body: body.to_string(),
        },
        404 => HttpResponse {
            code: 404,
            message: "NOT FOUND".to_string(),
            body: body.to_string(),
        },
        code => HttpResponse {
            code,
            message: "ERROR".to_string(),
            body: body.to_string(),
        },
    }
}

fn split_request(stream: &TcpStream) -> Vec<String> {
    let buf_reader = BufReader::new(stream);
    let request_lines = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    dbg!("{:#?}", &request_lines);
    request_lines
}
fn parse_request(http_request_lines: Vec<String>) -> Result<HttpRequest, HttpResponse> {
    let parse_error = error(400, "could not parse request");

    let mut http_request_line_iter = http_request_lines.iter();
    let request_line = match http_request_line_iter.next() {
        Some(line) => line,
        None => return Err(parse_error),
    };

    let mut request_line_iter = request_line.split_whitespace();

    let method = match request_line_iter.next() {
        Some(method @ ("GET" | "POST" | "PATCH" | "DELETE")) => method,
        _ => return Err(parse_error),
    };

    let uri = request_line_iter.next();

    let mut body = String::from("");
    let header_vector: Vec<(String, String)> = http_request_line_iter
        .filter_map(|request_line| {
            let header_values: Vec<&str> = match request_line.find(":") {
                Some(_) => request_line.split(":").collect(),
                None => {
                    body.push_str(&request_line);
                    return None;
                }
            };
            let header = header_values.get(0);
            let value = header_values.get(1);
            match (header, value) {
                (Some(header), Some(value)) => {
                    Some((header.trim().to_string(), value.trim().to_string()))
                }
                _ => None,
            }
        })
        .collect();

    let mut headers = HashMap::new();

    for (header, value) in header_vector {
        headers.insert(header, value);
    }

    dbg!("{:#?}", &headers);

    match (method, uri) {
        (method, Some(uri)) => Ok(HttpRequest {
            method: method.to_string(),
            uri: uri.to_string(),
            headers,
            body,
        }),
        (_, _) => Err(parse_error),
    }
}

fn send_response(mut stream: TcpStream, response: HttpResponse) {
    let status_line = format!("HTTP/1.1 {} {}\r\n\r\n", response.code, response.message);
    let response = format!("{status_line}\r\n{}", response.body);
    stream.write_all(response.as_bytes()).unwrap();
}

fn handle_connection(stream: TcpStream) {
    let request_lines = split_request(&stream);

    let request = match parse_request(request_lines) {
        Ok(request) => request,
        Err(response) => return send_response(stream, response),
    };
    let HttpRequest {
        method,
        uri,
        headers,
        body,
    } = request;

    match (method.as_str(), uri.as_str()) {
        ("GET", "/") => send_response(
            stream,
            hello_world(HttpRequest {
                method,
                uri,
                headers,
                body,
            }),
        ),
        _ => send_response(
            stream,
            error(404, &fs::read_to_string("./src/404.html").unwrap()),
        ),
    }
}
