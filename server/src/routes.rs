use std::fs;

use crate::{json::JsonValue, response::HttpCode, HttpRequest, HttpResponse};

pub fn hello_world(_: HttpRequest) -> HttpResponse {
    HttpResponse {
        code: HttpCode::Ok,
        body: Box::new(fs::read_to_string("./src/hello.html").unwrap()),
    }
}

pub fn test_post(request: HttpRequest) -> HttpResponse {
    HttpResponse {
        code: HttpCode::Ok,
        body: Box::new(request.body.unwrap_or(JsonValue::Array(vec![]))),
    }
}
