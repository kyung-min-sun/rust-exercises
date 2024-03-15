use std::{io::Write, net::TcpStream};

use crate::json::JsonValue;

pub struct HttpResponse {
    pub code: u32,
    pub message: String,
    pub body: Box<dyn ToString>,
}

pub fn http_error(code: u32, error_message: &str) -> HttpResponse {
    match code {
        400 => HttpResponse {
            code: 400,
            message: "BAD REQUEST".to_string(),
            body: Box::new(JsonValue::String(error_message.to_string())),
        },
        404 => HttpResponse {
            code: 404,
            message: "NOT FOUND".to_string(),
            body: Box::new(JsonValue::String(error_message.to_string())),
        },
        code => HttpResponse {
            code,
            message: "ERROR".to_string(),
            body: Box::new(JsonValue::String(error_message.to_string())),
        },
    }
}

pub fn send_response(mut stream: TcpStream, response: HttpResponse) {
    let status_line = format!("HTTP/1.1 {} {}\r\n\r\n", response.code, response.message);
    let response = format!("{status_line}\r\n{}", response.body.to_string());
    stream.write_all(response.as_bytes()).unwrap();
}
