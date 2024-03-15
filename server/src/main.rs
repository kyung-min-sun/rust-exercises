use std::{
    collections::HashMap,
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

struct HttpRequest<'a> {
    pub method: &'a str,
    pub uri: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

struct HttpResponse {
    pub code: u32,
    pub message: String,
    pub body: String,
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

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}
fn split_request(stream: &TcpStream) -> Vec<String> {
    let buf_reader = BufReader::new(stream);
    buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect()
}
fn parse_request<'a>(http_request_lines: Vec<String>) -> Result<HttpRequest<'a>, HttpResponse> {
    let parse_error = error(400, "could not parse request");

    let request_line = match http_request_lines.get(0) {
        Some(line) => line,
        None => return Err(parse_error),
    };

    let mut request_line_iter = request_line.split_whitespace();

    let method = match request_line_iter.next() {
        Some("GET") => "GET",
        Some("POST") => "POST",
        Some("PATCH") => "PATCH",
        Some("DELETE") => "DELETE",
        _ => return Err(parse_error),
    };

    let uri = request_line_iter.next();
    let headers: HashMap<String, String> = HashMap::new();

    match (method, uri) {
        (method, Some(uri)) => Ok(HttpRequest {
            method,
            uri: uri.to_string(),
            headers,
            body: "".to_string(),
        }),
        (_, _) => Err(parse_error),
    }
}

fn send_response(mut stream: TcpStream, response: HttpResponse) {
    let status_line = format!("HTTP/1.1 {} {}\r\n\r\n", response.code, response.message);
    let response = format!("{status_line}\r\n{}", response.body);
    stream.write_all(response.as_bytes()).unwrap();
}

fn handle_connection(mut stream: TcpStream) {
    let request_lines = split_request(&stream);
    let HttpRequest { method, uri, .. } = match parse_request(request_lines) {
        Ok(request) => request,
        Err(response) => return send_response(stream, response),
    };

    match (method, uri.as_str()) {
        ("GET", "/") => send_response(
            stream,
            HttpResponse {
                code: 200,
                message: "Ok".to_string(),
                body: fs::read_to_string("./src/hello.html").unwrap(),
            },
        ),
        _ => send_response(
            stream,
            error(404, &fs::read_to_string("./src/404.html").unwrap()),
        ),
    }
}
