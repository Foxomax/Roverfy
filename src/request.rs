use std::collections::HashMap;

#[derive(Debug)]
pub struct Request {
    method: String,
    url: String,
    headers: Vec<(String, String)>,
    body: HashMap<String, String>,
    protocol: String,
}

impl Request {
    pub fn new(request: &str) -> Self {
        let (method, url, body, headers, protocol) = Self::parser(request);
        Self {
            method,
            url,
            headers,
            body,
            protocol,
        }
    }

    fn parser(request: &str) -> (String, String, HashMap<String, String>, Vec<(String, String)>, String) {
        let parts: Vec<&str> = request.split("\n\n").collect();
        let lines: Vec<&str> = parts[0].split("\n").collect();
        let start_line: Vec<&str> = lines[0].split_whitespace().collect();

        let mut headers: Vec<(String, String)> = Vec::new();
        for line in &lines[1..] {
            if let Some((key, value)) = line.split_once(':') {
                headers.push((key.trim().to_string(), value.trim().to_string()));
            }
        }

        let method = start_line[0].to_string();
        let url = start_line[1].to_string();
        let protocol = start_line[2].to_string();

        // todo: parse body depending on the type of request
        let body = HashMap::new();
        
        
        (method, url, body, headers, protocol)
    }

    
}