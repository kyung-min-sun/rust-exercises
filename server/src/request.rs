use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    net::TcpStream,
};

use crate::{
    json::{parse_json, JsonValue},
    response::{self, HttpResponse},
};

pub struct HttpRequest {
    pub method: String,
    pub uri: String,
    pub headers: HashMap<String, String>,
    pub body: Option<JsonValue>,
}

pub fn split_request(stream: &TcpStream) -> Vec<String> {
    let buf_reader = BufReader::new(stream);
    let request_lines = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    dbg!("{:#?}", &request_lines);
    request_lines
}

pub fn parse_request(http_request_lines: Vec<String>) -> Result<HttpRequest, HttpResponse> {
    let parse_error = response::http_error(400, "could not parse request");

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

    match (method, uri) {
        (method, Some(uri)) => Ok(HttpRequest {
            method: method.to_string(),
            uri: uri.to_string(),
            headers,
            body: parse_json(&body),
        }),
        (_, _) => Err(parse_error),
    }
}
