use std::collections::HashMap;
use serde_json::Value;

#[derive(Debug)]
pub enum Body {
    Form(HashMap<String, String>),
    Json(Value),
    Empty,
}

#[derive(Debug)]
pub struct Request {
    method: String,
    url: String,
    headers: Vec<(String, String)>,
    body: Body,
    protocol: String,
}

impl Request {
    pub fn new(request: &str) -> Self {
        let (method, url, headers, body, protocol) = Self::parser(request);
        Self { method, url, headers, body, protocol }
    }

    fn parser(
        request: &str
    ) -> (String, String, Vec<(String, String)>, Body, String) {
        let parts: Vec<&str> = request.split("\r\n\r\n").collect();
        let header_section = parts.get(0).unwrap_or(&"");
        let raw_body = parts.get(1).unwrap_or(&"");

        let mut lines = header_section.lines();
        let start_line = lines.next().unwrap_or("").split_whitespace().collect::<Vec<_>>();
        let method = start_line.get(0).unwrap_or(&"").to_string();
        let url = start_line.get(1).unwrap_or(&"").to_string();
        let protocol = start_line.get(2).unwrap_or(&"").to_string();

        let headers: Vec<(String, String)> = lines
            .filter_map(|line| line.split_once(':').map(|(k, v)| (k.trim().to_string(), v.trim().to_string())))
            .collect();

        let body = Self::parse_body(raw_body, &headers);

        (method, url, headers, body, protocol)
    }
    
    fn get_header<'a>(headers: &'a [(String, String)], key: &str) -> Option<&'a str> {
        headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(key))
            .map(|(_, v)| v.as_str())
    }

    fn parse_body(raw: &str, headers: &[(String, String)]) -> Body {
        match Self::get_header(headers, "Content-Type") {
            Some(ct) if ct.contains("application/json") => Self::parse_json_body(raw),
            Some(ct) if ct.contains("application/x-www-form-urlencoded") => Self::parse_form_body(raw),
            _ => Body::Empty,
        }
    }

    fn parse_form_body(raw: &str) -> Body {
        let mut map = HashMap::new();
        for pair in raw.split('&') {
            if let Some((k, v)) = pair.split_once('=') {
                map.insert(k.to_string(), v.to_string());
            }
        }
        Body::Form(map)
    }

    fn parse_json_body(raw: &str) -> Body {
        match serde_json::from_str::<Value>(raw) {
            Ok(json) => Body::Json(json),
            Err(_) => Body::Empty,
        }
    }

    pub fn get_form(&self, key: &str) -> Option<&String> {
        if let Body::Form(map) = &self.body {
            map.get(key)
        } else {
            None
        }
    }

    pub fn get_json(&self, pointer: &str) -> Option<&Value> {
        if let Body::Json(val) = &self.body {
            val.pointer(pointer)
        } else {
            None
        }
    }
    
    pub fn get_url(&self) -> &str {
        &self.url
    }
}
