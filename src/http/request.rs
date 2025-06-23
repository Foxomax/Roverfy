use std::collections::HashMap;
use serde_json::Value;
use crate::http::{ContentType, Method};

/// Represents the body content of an HTTP request.
/// 
/// This enum provides type-safe handling of different request body formats.
/// It supports form data, JSON, and empty bodies.
/// 
/// # Examples
/// 
/// ```
/// use roverfy::http::request::{Request, Body};
/// use std::collections::HashMap;
/// 
/// // Form data
/// let mut form_data = HashMap::new();
/// form_data.insert("name".to_string(), "John".to_string());
/// let body = Body::Form(form_data);
/// 
/// // JSON data
/// let json_body = Body::Json(serde_json::json!({"name": "John"}));
/// 
/// // Empty body
/// let empty_body = Body::Empty;
/// ```
#[derive(Debug, Clone)]
pub enum Body {
    /// Form-encoded data as key-value pairs
    Form(HashMap<String, String>),
    /// JSON data as a serde_json::Value
    Json(Value),
    /// Empty or no body content
    Empty,
}

/// Represents an HTTP request with all its components.
/// 
/// This struct encapsulates a complete HTTP request including method, URL,
/// headers, body, and protocol information. It provides methods for parsing
/// raw HTTP request strings and accessing request data in a type-safe manner.
/// 
/// # Examples
/// 
/// ```
/// use roverfy::http::request::Request;
/// 
/// let raw_request = "POST /api/users HTTP/1.1\r\n\
///                    Content-Type: application/json\r\n\
///                    \r\n\
///                    {\"name\": \"John\", \"age\": 30}";
/// 
/// let request = Request::new(raw_request);
/// assert_eq!(request.get_url(), "/api/users");
/// assert_eq!(request.get_method().as_str(), "POST");
/// ```
#[derive(Debug, Clone)]
pub struct Request {
    method: Method,
    url: String,
    headers: Vec<(String, String)>,
    body: Body,
    protocol: String,
}

impl Request {
    /// Creates a new Request instance by parsing a raw HTTP request string.
    /// 
    /// This method parses the complete HTTP request including headers and body.
    /// It handles different content types and automatically parses the body
    /// based on the Content-Type header.
    /// 
    /// # Arguments
    /// 
    /// * `request` - The raw HTTP request string to parse
    /// 
    /// # Returns
    /// 
    /// A new `Request` instance with parsed data
    /// 
    /// # Examples
    /// 
    /// ```
    /// use roverfy::http::request::Request;
    /// 
    /// let raw_request = "GET /api/users HTTP/1.1\r\n\
    ///                    Host: example.com\r\n\
    ///                    \r\n";
    /// 
    /// let request = Request::new(raw_request);
    /// ```
    pub fn new(request: &str) -> Self {
        let (method, url, headers, body, protocol) = Self::parse_request(request);
        Self { method, url, headers, body, protocol }
    }

    /// Parses a complete HTTP request string into its components.
    /// 
    /// This method extracts the method, URL, headers, body, and protocol
    /// from a raw HTTP request string.
    /// 
    /// # Arguments
    /// 
    /// * `request` - The raw HTTP request string
    /// 
    /// # Returns
    /// 
    /// A tuple containing (Method, URL, Headers, Body, Protocol)
    fn parse_request(request: &str) -> (Method, String, Vec<(String, String)>, Body, String) {
        let (header_section, raw_body) = Self::split_request(request);
        let mut lines = header_section.lines();

        let start_line = Self::parse_start_line(lines.next().unwrap_or(""));
        let method = Self::parse_method_string(&start_line.0).unwrap_or(Method::GET);
        let url = start_line.1;
        let protocol = start_line.2;

        let headers = Self::parse_headers(lines);
        let body = Self::parse_body(raw_body, &headers);

        (method, url, headers, body, protocol)
    }

    /// Splits an HTTP request into header and body sections.
    /// 
    /// HTTP requests are separated by a double CRLF (`\r\n\r\n`).
    /// This method splits the request at that boundary.
    /// 
    /// # Arguments
    /// 
    /// * `request` - The complete HTTP request string
    /// 
    /// # Returns
    /// 
    /// A tuple containing (header_section, body_section)
    fn split_request(request: &str) -> (&str, &str) {
        let parts: Vec<&str> = request.split("\r\n\r\n").collect();
        let header_section = parts.get(0).unwrap_or(&"");
        let raw_body = parts.get(1).unwrap_or(&"");
        (header_section, raw_body)
    }

    /// Parses the HTTP request start line (method, URL, protocol).
    /// 
    /// The start line format is: `METHOD URL PROTOCOL`
    /// 
    /// # Arguments
    /// 
    /// * `line` - The start line string
    /// 
    /// # Returns
    /// 
    /// A tuple containing (method, url, protocol)
    fn parse_start_line(line: &str) -> (String, String, String) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        (
            parts.get(0).unwrap_or(&"").to_string(),
            parts.get(1).unwrap_or(&"").to_string(),
            parts.get(2).unwrap_or(&"").to_string(),
        )
    }

    /// Parses HTTP headers from an iterator of header lines.
    /// 
    /// Headers are in the format `Name: Value` and are parsed into
    /// key-value pairs.
    /// 
    /// # Arguments
    /// 
    /// * `lines` - Iterator over header lines
    /// 
    /// # Returns
    /// 
    /// Vector of (header_name, header_value) pairs
    fn parse_headers<'a, I>(lines: I) -> Vec<(String, String)>
    where
        I: Iterator<Item = &'a str>,
    {
        lines
            .filter(|line| !line.is_empty())
            .filter_map(|line| {
                line.split_once(':')
                    .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
            })
            .collect()
    }
    
    /// Retrieves a header value by name (case-insensitive).
    /// 
    /// # Arguments
    /// 
    /// * `headers` - Reference to header collection
    /// * `key` - The header name to search for
    /// 
    /// # Returns
    /// 
    /// `Some(&str)` if the header is found, `None` otherwise
    fn find_header<'a>(headers: &'a [(String, String)], key: &str) -> Option<&'a str> {
        headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(key))
            .map(|(_, v)| v.as_str())
    }
    
    /// Converts a string to a Method enum.
    /// 
    /// # Arguments
    /// 
    /// * `method` - The HTTP method string
    /// 
    /// # Returns
    /// 
    /// `Some(Method)` if valid, `None` otherwise
    fn parse_method_string(method: &str) -> Option<Method> {
        Method::from_str(method)
    }

    /// Parses the request body based on the Content-Type header.
    /// 
    /// This method automatically determines the body format and parses it
    /// accordingly. Supported formats include JSON and form-encoded data.
    /// 
    /// # Arguments
    /// 
    /// * `raw` - The raw body string
    /// * `headers` - The request headers
    /// 
    /// # Returns
    /// 
    /// A `Body` enum variant representing the parsed body
    fn parse_body(raw: &str, headers: &[(String, String)]) -> Body {
        if raw.is_empty() {
            return Body::Empty;
        }

        if let Some(ct_str) = Self::find_header(headers, "Content-Type") {
            if let Some(content_type) = ContentType::parse_str(ct_str) {
                match content_type {
                    ContentType::Json => Self::parse_json_body(raw),
                    ContentType::FormUrlEncoded => Self::parse_form_body(raw),
                    _ => Body::Empty,
                }
            } else {
                Body::Empty
            }
        } else {
            Body::Empty
        }
    }

    /// Parses form-encoded body data.
    /// 
    /// Form data is in the format `key1=value1&key2=value2`.
    /// 
    /// # Arguments
    /// 
    /// * `raw` - The raw form data string
    /// 
    /// # Returns
    /// 
    /// A `Body::Form` variant with parsed key-value pairs
    fn parse_form_body(raw: &str) -> Body {
        let mut map = HashMap::new();
        for pair in raw.split('&') {
            if let Some((k, v)) = pair.split_once('=') {
                // URL decode the values
                let decoded_key = urlencoding::decode(k).unwrap_or_else(|_| k.to_string().into()).to_string();
                let decoded_value = urlencoding::decode(v).unwrap_or_else(|_| v.to_string().into()).to_string();
                map.insert(decoded_key, decoded_value);
            }
        }
        Body::Form(map)
    }

    /// Parses JSON body data.
    /// 
    /// Attempts to parse the raw string as JSON. If parsing fails,
    /// returns an empty body.
    /// 
    /// # Arguments
    /// 
    /// * `raw` - The raw JSON string
    /// 
    /// # Returns
    /// 
    /// A `Body::Json` variant if parsing succeeds, `Body::Empty` otherwise
    fn parse_json_body(raw: &str) -> Body {
        match serde_json::from_str::<Value>(raw) {
            Ok(json) => Body::Json(json),
            Err(_) => Body::Empty,
        }
    }

    /// Retrieves a form field value by key.
    /// 
    /// This method only works if the request body is form-encoded.
    /// 
    /// # Arguments
    /// 
    /// * `key` - The form field key
    /// 
    /// # Returns
    /// 
    /// `Some(&String)` if the field exists, `None` otherwise
    /// 
    /// # Examples
    /// 
    /// ```
    /// use roverfy::http::request::Request;
    /// 
    /// let raw_request = "POST /submit HTTP/1.1\r\n\
    ///                    Content-Type: application/x-www-form-urlencoded\r\n\
    ///                    \r\n\
    ///                    name=John&age=30";
    /// 
    /// let request = Request::new(raw_request);
    /// assert_eq!(request.get_form("name"), Some(&"John".to_string()));
    /// ```
    pub fn get_form(&self, key: &str) -> Option<&String> {
        if let Body::Form(map) = &self.body {
            map.get(key)
        } else {
            None
        }
    }

    /// Retrieves a JSON field value using a JSON pointer.
    /// 
    /// This method only works if the request body is JSON.
    /// JSON pointers use the format `/path/to/field`.
    /// 
    /// # Arguments
    /// 
    /// * `pointer` - The JSON pointer path
    /// 
    /// # Returns
    /// 
    /// `Some(&Value)` if the field exists, `None` otherwise
    /// 
    /// # Examples
    /// 
    /// ```
    /// use roverfy::http::request::Request;
    /// 
    /// let raw_request = "POST /api/users HTTP/1.1\r\n\
    ///                    Content-Type: application/json\r\n\
    ///                    \r\n\
    ///                    {\"user\": {\"name\": \"John\", \"age\": 30}}";
    /// 
    /// let request = Request::new(raw_request);
    /// let name = request.get_json("/user/name");
    /// assert!(name.is_some());
    /// ```
    pub fn get_json(&self, pointer: &str) -> Option<&Value> {
        if let Body::Json(val) = &self.body {
            val.pointer(pointer)
        } else {
            None
        }
    }
    
    /// Returns the request URL.
    /// 
    /// # Returns
    /// 
    /// A string slice containing the request URL
    pub fn get_url(&self) -> &str {
        &self.url
    }

    /// Returns the HTTP method.
    /// 
    /// # Returns
    /// 
    /// A reference to the request's HTTP method
    pub fn get_method(&self) -> &Method {
        &self.method
    }

    /// Returns the HTTP protocol version.
    /// 
    /// # Returns
    /// 
    /// A string slice containing the protocol version (e.g., "HTTP/1.1")
    pub fn get_protocol(&self) -> &str {
        &self.protocol
    }

    /// Returns all request headers.
    /// 
    /// # Returns
    /// 
    /// A slice of (header_name, header_value) pairs
    pub fn get_headers(&self) -> &[(String, String)] {
        &self.headers
    }

    /// Retrieves a specific header value by name (case-insensitive).
    /// 
    /// # Arguments
    /// 
    /// * `name` - The header name to search for
    /// 
    /// # Returns
    /// 
    /// `Some(&str)` if the header is found, `None` otherwise
    /// 
    /// # Examples
    /// 
    /// ```
    /// use roverfy::http::request::Request;
    /// 
    /// let raw_request = "GET /api/users HTTP/1.1\r\n\
    ///                    Host: example.com\r\n\
    ///                    User-Agent: MyApp/1.0\r\n\
    ///                    \r\n";
    /// 
    /// let request = Request::new(raw_request);
    /// assert_eq!(request.get_header("Host"), Some("example.com"));
    /// assert_eq!(request.get_header("user-agent"), Some("MyApp/1.0"));
    /// ```
    pub fn get_header(&self, name: &str) -> Option<&str> {
        Self::find_header(&self.headers, name)
    }

    /// Returns the request body.
    /// 
    /// # Returns
    /// 
    /// A reference to the request body
    pub fn get_body(&self) -> &Body {
        &self.body
    }

    /// Checks if the request has a body.
    /// 
    /// # Returns
    /// 
    /// `true` if the request has a non-empty body, `false` otherwise
    pub fn has_body(&self) -> bool {
        !matches!(self.body, Body::Empty)
    }

    /// Checks if the request body is form-encoded.
    /// 
    /// # Returns
    /// 
    /// `true` if the body is form-encoded, `false` otherwise
    pub fn is_form(&self) -> bool {
        matches!(self.body, Body::Form(_))
    }

    /// Checks if the request body is JSON.
    /// 
    /// # Returns
    /// 
    /// `true` if the body is JSON, `false` otherwise
    pub fn is_json(&self) -> bool {
        matches!(self.body, Body::Json(_))
    }
}
