use super::super::contenttypes::ContentType;
use std::str::FromStr;
use crate::errors::ContentTypeParseError;

#[test]
fn test_content_type_as_str() {
    assert_eq!(ContentType::Html.as_str(), "text/html; charset=utf-8");
    assert_eq!(ContentType::Json.as_str(), "application/json; charset=utf-8");
    assert_eq!(ContentType::Png.as_str(), "image/png");
}

#[test]
fn test_content_type_from_str() {
    assert_eq!(ContentType::parse_str("application/json"), Some(ContentType::Json));
    assert_eq!(ContentType::parse_str("text/html"), Some(ContentType::Html));
    assert_eq!(ContentType::parse_str("image/jpeg"), Some(ContentType::Jpeg));
    assert_eq!(ContentType::parse_str("unknown/type"), None);
}

#[test]
fn test_content_type_case_insensitive() {
    assert_eq!(ContentType::parse_str("APPLICATION/JSON"), Some(ContentType::Json));
    assert_eq!(ContentType::parse_str("Text/Html"), Some(ContentType::Html));
}

#[test]
fn test_content_type_extension() {
    assert_eq!(ContentType::Html.extension(), "html");
    assert_eq!(ContentType::Json.extension(), "json");
    assert_eq!(ContentType::Png.extension(), "png");
}

#[test]
fn test_content_type_is_text() {
    assert!(ContentType::Html.is_text());
    assert!(ContentType::Json.is_text());
    assert!(ContentType::PlainText.is_text());
    assert!(!ContentType::Png.is_text());
    assert!(!ContentType::Pdf.is_text());
}

#[test]
fn test_content_type_is_image() {
    assert!(ContentType::Png.is_image());
    assert!(ContentType::Jpeg.is_image());
    assert!(ContentType::Gif.is_image());
    assert!(ContentType::Svg.is_image());
    assert!(!ContentType::Html.is_image());
    assert!(!ContentType::Json.is_image());
}

#[test]
fn test_display_trait() {
    assert_eq!(ContentType::Html.to_string(), "text/html; charset=utf-8");
    assert_eq!(ContentType::Json.to_string(), "application/json; charset=utf-8");
}

#[test]
fn test_from_str_trait() {
    assert_eq!(ContentType::from_str("application/json").unwrap(), ContentType::Json);
    assert!(ContentType::from_str("invalid/type").is_err());
}

#[test]
fn test_parse_error() {
    let error = ContentType::from_str("invalid/type").unwrap_err();
    assert_eq!(error.input, "invalid/type");
    assert_eq!(error.to_string(), "Failed to parse content type from 'invalid/type'");
} 