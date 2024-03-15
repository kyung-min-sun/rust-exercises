use std::fs;

use crate::{HttpRequest, HttpResponse};

pub fn hello_world(_: HttpRequest) -> HttpResponse {
    HttpResponse {
        code: 200,
        message: "Ok".to_string(),
        body: Box::new(fs::read_to_string("./src/hello.html").unwrap()),
    }
}
