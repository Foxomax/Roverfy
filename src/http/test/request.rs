//! Module for HTTP request parsing tests.

use crate::http::request::{Body, Request};
use crate::http::Method;
use serde_json::json;

#[test]
fn test_parse_simple_get_request() {
    let raw_request = "GET /test/path HTTP/1.1\r\n\
                       Host: example.com\r\n\
                       User-Agent: TestClient/1.0\r\n\
                       \r\n";

    let request = Request::new(raw_request);

    assert_eq!(*request.get_method(), Method::GET);
    assert_eq!(request.get_url(), "/test/path");
    assert_eq!(request.get_protocol(), "HTTP/1.1");
    assert_eq!(request.get_header("Host"), Some("example.com"));
    assert!(!request.has_body());
}

#[test]
fn test_parse_post_request_with_json_body() {
    let raw_request = "POST /api/data HTTP/1.1\r\n\
                       Host: an-api.com\r\n\
                       Content-Type: application/json\r\n\
                       Content-Length: 27\r\n\
                       \r\n\
                       {\"name\":\"John\",\"age\":30}";

    let request = Request::new(raw_request);

    assert_eq!(*request.get_method(), Method::POST);
    assert_eq!(request.get_url(), "/api/data");
    assert!(request.has_body());
    assert!(request.is_json());
    assert!(!request.is_form());

    if let Body::Json(json_body) = request.get_body() {
        assert_eq!(json_body, &json!({"name": "John", "age": 30}));
    } else {
        panic!("Expected JSON body, but found something else.");
    }
}

#[test]
fn test_parse_post_request_with_form_body() {
    let raw_request = "POST /submit-form HTTP/1.1\r\n\
                       Host: web.app\r\n\
                       Content-Type: application/x-www-form-urlencoded\r\n\
                       Content-Length: 27\r\n\
                       \r\n\
                       field1=value1&field2=value2";

    let request = Request::new(raw_request);
    assert_eq!(*request.get_method(), Method::POST);
    assert!(request.has_body());
    assert!(request.is_form());
    assert!(!request.is_json());

    assert_eq!(request.get_form("field1"), Some(&"value1".to_string()));
    assert_eq!(request.get_form("field2"), Some(&"value2".to_string()));
    assert_eq!(request.get_form("nonexistent"), None);
}

#[test]
fn test_get_header_is_case_insensitive() {
    let raw_request = "GET /resource HTTP/1.1\r\n\
                       Content-Type: text/plain\r\n\
                       x-custom-header: MyValue\r\n\
                       \r\n";

    let request = Request::new(raw_request);
    assert_eq!(request.get_header("content-type"), Some("text/plain"));
    assert_eq!(request.get_header("X-Custom-Header"), Some("MyValue"));
} 