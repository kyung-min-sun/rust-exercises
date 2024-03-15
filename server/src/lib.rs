mod json;
mod request;
mod response;
mod routes;
mod thread_pool;

use request::HttpRequest;
use response::HttpResponse;
use routes::*;
use std::{
    fs,
    net::{TcpListener, TcpStream},
};

use thread_pool::ThreadPool;

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

fn handle_connection(stream: TcpStream) {
    let request_lines = request::split_request(&stream);

    let request = match request::parse_request(request_lines) {
        Ok(request) => request,
        Err(response) => return response::send_response(stream, response),
    };

    let HttpRequest {
        method,
        uri,
        headers,
        body,
    } = request;

    match (method.as_str(), uri.as_str()) {
        ("GET", "/") => response::send_response(
            stream,
            hello_world(HttpRequest {
                method,
                uri,
                headers,
                body,
            }),
        ),
        _ => response::send_response(
            stream,
            response::http_error(404, &fs::read_to_string("./src/404.html").unwrap()),
        ),
    }
}
