use crate::http::{StatusCode, ContentType};

/// Builder trait for constructing HTTP responses.
/// 
/// This trait provides a standardized way to build HTTP responses
/// with consistent formatting and proper HTTP protocol compliance.
/// 
/// # Examples
/// 
/// ```
/// use roverfy::http::{HttpResponseBuilder, ResponseBuilder, StatusCode};
/// 
/// let builder = HttpResponseBuilder;
/// let headers = vec![("Content-Type".to_string(), "text/html".to_string())];
/// let body = "<html><body>Hello World!</body></html>".to_string();
/// 
/// let response_bytes = builder.build(StatusCode::OK, headers, body);
/// ```
pub trait ResponseBuilder {
    /// Builds an HTTP response from the given components.
    /// 
    /// # Arguments
    /// 
    /// * `status_code` - The HTTP status code for the response
    /// * `headers` - Vector of (header_name, header_value) pairs
    /// * `body` - The response body content
    /// 
    /// # Returns
    /// 
    /// A byte vector containing the complete HTTP response
    /// 
    /// # Examples
    /// 
    /// ```
    /// use roverfy::http::{HttpResponseBuilder, ResponseBuilder, StatusCode};
    /// 
    /// let builder = HttpResponseBuilder;
    /// let response = builder.build(
    ///     StatusCode::OK,
    ///     vec![("Content-Type".to_string(), "text/plain".to_string())],
    ///     "Hello, World!".to_string()
    /// );
    /// 
    /// assert!(!response.is_empty());
    /// ```
    fn build(
        &self,
        status_code: StatusCode,
        headers: Vec<(String, String)>,
        body: String
    ) -> Vec<u8>;
}

/// Default implementation of ResponseBuilder for HTTP responses.
/// 
/// This struct provides a concrete implementation of the ResponseBuilder trait
/// for creating standard HTTP responses.
#[derive(Debug, Clone, Copy)]
pub struct HttpResponseBuilder;

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

/// Represents a complete HTTP response with status, headers, and body.
/// 
/// This struct encapsulates all components of an HTTP response and provides
/// methods for creating and serializing responses according to HTTP standards.
/// 
/// # Examples
/// 
/// ```
/// use roverfy::http::{HttpResponse, StatusCode};
/// 
/// let response = HttpResponse::new(
///     StatusCode::OK,
///     vec![("Content-Type".to_string(), "text/html".to_string())],
///     "<html><body>Hello World!</body></html>".to_string()
/// );
/// 
/// let response_bytes = response.to_bytes();
/// assert!(!response_bytes.is_empty());
/// ```
#[derive(Debug, Clone)]
pub struct HttpResponse {
    /// The HTTP status code for this response
    pub status_code: StatusCode,
    /// Vector of (header_name, header_value) pairs
    pub headers: Vec<(String, String)>,
    /// The response body content
    pub body: String,
}

impl HttpResponse {
    /// Creates a new HTTP response with the specified components.
    /// 
    /// # Arguments
    /// 
    /// * `status_code` - The HTTP status code
    /// * `headers` - Vector of header key-value pairs
    /// * `body` - The response body content
    /// 
    /// # Returns
    /// 
    /// A new `HttpResponse` instance
    /// 
    /// # Examples
    /// 
    /// ```
    /// use roverfy::http::{HttpResponse, StatusCode, ContentType};
    /// 
    /// let response = HttpResponse::new(
    ///     StatusCode::NotFound,
    ///     vec![("Content-Type".to_string(), ContentType::PlainText.as_str().to_string())],
    ///     "Page not found".to_string()
    /// );
    /// 
    /// assert_eq!(response.status_code, StatusCode::NotFound);
    /// ```
    pub fn new(status_code: StatusCode, headers: Vec<(String, String)>, body: String) -> Self {
        Self {
            status_code,
            headers,
            body,
        }
    }

    /// Converts the HTTP response to a byte vector for network transmission.
    /// 
    /// This method formats the response according to HTTP/1.1 standards,
    /// including the status line, headers, and body with proper line endings.
    /// 
    /// # Returns
    /// 
    /// A byte vector containing the complete HTTP response
    /// 
    /// # Examples
    /// 
    /// ```
    /// use roverfy::http::{HttpResponse, StatusCode};
    /// 
    /// let response = HttpResponse::new(
    ///     StatusCode::OK,
    ///     vec![("Content-Type".to_string(), "text/plain".to_string())],
    ///     "Hello, World!".to_string()
    /// );
    /// 
    /// let bytes = response.to_bytes();
    /// let response_string = String::from_utf8_lossy(&bytes);
    /// assert!(response_string.contains("HTTP/1.1 200 OK"));
    /// ```
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut response = format!("{}", self.status_code.as_response());
        
        // Add headers
        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }
        
        // Add empty line to separate headers from body
        response.push_str("\r\n");
        
        // Add body
        response.push_str(&self.body);
        
        response.into_bytes()
    }

    /// Returns the HTTP status code.
    /// 
    /// # Returns
    /// 
    /// A reference to the response's status code
    pub fn get_status_code(&self) -> &StatusCode {
        &self.status_code
    }

    /// Returns all response headers.
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
    /// use roverfy::http::{HttpResponse, StatusCode};
    /// 
    /// let response = HttpResponse::new(
    ///     StatusCode::OK,
    ///     vec![("Content-Type".to_string(), "text/html".to_string())],
    ///     "".to_string()
    /// );
    /// 
    /// assert_eq!(response.get_header("Content-Type"), Some("text/html"));
    /// assert_eq!(response.get_header("content-type"), Some("text/html"));
    /// ```
    pub fn get_header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str())
    }

    /// Returns the response body.
    /// 
    /// # Returns
    /// 
    /// A string slice containing the response body
    pub fn get_body(&self) -> &str {
        &self.body
    }

    /// Checks if the response has a body.
    /// 
    /// # Returns
    /// 
    /// `true` if the response has a non-empty body, `false` otherwise
    pub fn has_body(&self) -> bool {
        !self.body.is_empty()
    }

    /// Adds a header to the response.
    /// 
    /// If a header with the same name already exists, it will be replaced.
    /// 
    /// # Arguments
    /// 
    /// * `name` - The header name
    /// * `value` - The header value
    /// 
    /// # Examples
    /// 
    /// ```
    /// use roverfy::http::{HttpResponse, StatusCode};
    /// 
    /// let mut response = HttpResponse::new(
    ///     StatusCode::OK,
    ///     vec![],
    ///     "".to_string()
    /// );
    /// 
    /// response.add_header("Content-Type", "text/html");
    /// assert_eq!(response.get_header("Content-Type"), Some("text/html"));
    /// ```
    pub fn add_header(&mut self, name: &str, value: &str) {
        // Remove existing header with same name (case-insensitive)
        self.headers.retain(|(k, _)| !k.eq_ignore_ascii_case(name));
        
        // Add new header
        self.headers.push((name.to_string(), value.to_string()));
    }

    /// Sets the response body.
    /// 
    /// # Arguments
    /// 
    /// * `body` - The new body content
    /// 
    /// # Examples
    /// 
    /// ```
    /// use roverfy::http::{HttpResponse, StatusCode};
    /// 
    /// let mut response = HttpResponse::new(
    ///     StatusCode::OK,
    ///     vec![],
    ///     "".to_string()
    /// );
    /// 
    /// response.set_body("New content");
    /// assert_eq!(response.get_body(), "New content");
    /// ```
    pub fn set_body(&mut self, body: &str) {
        self.body = body.to_string();
    }

    /// Creates a simple text response with the given status code and message.
    /// 
    /// This is a convenience method for creating basic text responses.
    /// 
    /// # Arguments
    /// 
    /// * `status_code` - The HTTP status code
    /// * `message` - The response message
    /// 
    /// # Returns
    /// 
    /// A new `HttpResponse` with text/plain content type
    /// 
    /// # Examples
    /// 
    /// ```
    /// use roverfy::http::{HttpResponse, StatusCode};
    /// 
    /// let response = HttpResponse::text(StatusCode::NotFound, "Page not found");
    /// assert_eq!(response.get_header("Content-Type"), Some("text/plain; charset=utf-8"));
    /// assert_eq!(response.get_body(), "Page not found");
    /// ```
    pub fn text(status_code: StatusCode, message: &str) -> Self {
        Self::new(
            status_code,
            vec![("Content-Type".to_string(), ContentType::PlainText.as_str().to_string())],
            message.to_string()
        )
    }

    /// Creates an HTML response with the given status code and content.
    /// 
    /// This is a convenience method for creating HTML responses.
    /// 
    /// # Arguments
    /// 
    /// * `status_code` - The HTTP status code
    /// * `html` - The HTML content
    /// 
    /// # Returns
    /// 
    /// A new `HttpResponse` with text/html content type
    /// 
    /// # Examples
    /// 
    /// ```
    /// use roverfy::http::{HttpResponse, StatusCode};
    /// 
    /// let response = HttpResponse::html(StatusCode::OK, "<h1>Hello World</h1>");
    /// assert_eq!(response.get_header("Content-Type"), Some("text/html; charset=utf-8"));
    /// ```
    pub fn html(status_code: StatusCode, html: &str) -> Self {
        Self::new(
            status_code,
            vec![("Content-Type".to_string(), ContentType::Html.as_str().to_string())],
            html.to_string()
        )
    }

    /// Creates a JSON response with the given status code and data.
    /// 
    /// This is a convenience method for creating JSON responses.
    /// 
    /// # Arguments
    /// 
    /// * `status_code` - The HTTP status code
    /// * `json` - The JSON string content
    /// 
    /// # Returns
    /// 
    /// A new `HttpResponse` with application/json content type
    /// 
    /// # Examples
    /// 
    /// ```
    /// use roverfy::http::{HttpResponse, StatusCode};
    /// 
    /// let response = HttpResponse::json(StatusCode::OK, r#"{"message": "success"}"#);
    /// assert_eq!(response.get_header("Content-Type"), Some("application/json; charset=utf-8"));
    /// ```
    pub fn json(status_code: StatusCode, json: &str) -> Self {
        Self::new(
            status_code,
            vec![("Content-Type".to_string(), ContentType::Json.as_str().to_string())],
            json.to_string()
        )
    }
}