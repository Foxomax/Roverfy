
use crate::http::StatusCode;
pub struct HttpResponseBuilder;
pub trait ResponseBuilder {
    fn build(
        &self,
        status_code: StatusCode,
        headers: Vec<(String, String)>,
        body: String
    ) -> Vec<u8>;
}

impl ResponseBuilder for HttpResponseBuilder {
    fn build(
        &self,
        status_code: StatusCode,
        headers: Vec<(String, String)>,
        body: String
    ) -> Vec<u8> {
        HttpResponse::new(status_code, headers, body).to_bytes()
    }
}

pub struct HttpResponse {
    pub status_code: StatusCode,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

impl HttpResponse {
    pub fn new(status_code: StatusCode, headers: Vec<(String, String)>, body: String) -> Self {
        Self {
            status_code,
            headers,
            body,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut response = format!("HTTP/1.1 {:?}\r\n", self.status_code);
        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }
        response.push_str("\r\n");
        response.push_str(&self.body);
        response.into_bytes()
    }
}