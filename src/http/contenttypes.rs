use std::fmt;
use std::str::FromStr;
use crate::errors::ContentTypeParseError;
/// Represents HTTP content types (MIME types) commonly used in web applications.
/// 
/// This enum provides a type-safe way to handle content types and their string representations.
/// Each variant corresponds to a specific MIME type with appropriate charset encoding.
/// 
/// # Examples
/// 
/// ```
/// use roverfy::http::ContentType;
/// 
/// let html = ContentType::Html;
/// assert_eq!(html.as_str(), "text/html; charset=utf-8");
/// 
/// let json = ContentType::parse_str("application/json").unwrap();
/// assert_eq!(json, ContentType::Json);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContentType {
    /// HTML content type: `text/html; charset=utf-8`
    Html,
    /// JSON content type: `application/json; charset=utf-8`
    Json,
    /// Plain text content type: `text/plain; charset=utf-8`
    PlainText,
    /// XML content type: `application/xml; charset=utf-8`
    Xml,
    /// Form URL encoded content type: `application/x-www-form-urlencoded; charset=utf-8`
    FormUrlEncoded,
    /// CSS content type: `text/css; charset=utf-8`
    Css,
    /// JavaScript content type: `application/javascript; charset=utf-8`
    JavaScript,
    /// PNG image content type: `image/png`
    Png,
    /// JPEG image content type: `image/jpeg`
    Jpeg,
    /// GIF image content type: `image/gif`
    Gif,
    /// SVG image content type: `image/svg+xml`
    Svg,
    /// PDF document content type: `application/pdf`
    Pdf,
    /// ZIP archive content type: `application/zip`
    Zip,
    /// Binary content type: `application/octet-stream`
    Binary,
}

impl ContentType {
    /// Returns the string representation of the content type.
    /// 
    /// This method returns the full MIME type string including charset where appropriate.
    /// 
    /// # Returns
    /// 
    /// A string slice containing the MIME type representation.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use roverfy::http::ContentType;
    /// 
    /// assert_eq!(ContentType::Html.as_str(), "text/html; charset=utf-8");
    /// assert_eq!(ContentType::Json.as_str(), "application/json; charset=utf-8");
    /// assert_eq!(ContentType::Png.as_str(), "image/png");
    /// ```
    pub fn as_str(&self) -> &str {
        match self {
            ContentType::Html => "text/html; charset=utf-8",
            ContentType::Json => "application/json; charset=utf-8",
            ContentType::PlainText => "text/plain; charset=utf-8",
            ContentType::Xml => "application/xml; charset=utf-8",
            ContentType::FormUrlEncoded => "application/x-www-form-urlencoded; charset=utf-8",
            ContentType::Css => "text/css; charset=utf-8",
            ContentType::JavaScript => "application/javascript; charset=utf-8",
            ContentType::Png => "image/png",
            ContentType::Jpeg => "image/jpeg",
            ContentType::Gif => "image/gif",
            ContentType::Svg => "image/svg+xml",
            ContentType::Pdf => "application/pdf",
            ContentType::Zip => "application/zip",
            ContentType::Binary => "application/octet-stream",
        }
    }

    /// Attempts to parse a content type from a string.
    /// 
    /// This method performs a case-insensitive partial match on the input string.
    /// It will match if the input contains the MIME type substring.
    /// 
    /// # Arguments
    /// 
    /// * `s` - The string to parse
    /// 
    /// # Returns
    /// 
    /// `Some(ContentType)` if a match is found, `None` otherwise.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use roverfy::http::ContentType;
    /// 
    /// assert_eq!(ContentType::parse_str("application/json"), Some(ContentType::Json));
    /// assert_eq!(ContentType::parse_str("text/html"), Some(ContentType::Html));
    /// assert_eq!(ContentType::parse_str("unknown/type"), None);
    /// ```
    pub fn parse_str(s: &str) -> Option<Self> {
        let s_lower = s.to_lowercase();
        
        if s_lower.contains("application/json") {
            Some(ContentType::Json)
        } else if s_lower.contains("application/x-www-form-urlencoded") {
            Some(ContentType::FormUrlEncoded)
        } else if s_lower.contains("text/html") {
            Some(ContentType::Html)
        } else if s_lower.contains("text/plain") {
            Some(ContentType::PlainText)
        } else if s_lower.contains("application/xml") {
            Some(ContentType::Xml)
        } else if s_lower.contains("text/css") {
            Some(ContentType::Css)
        } else if s_lower.contains("application/javascript") || s_lower.contains("text/javascript") {
            Some(ContentType::JavaScript)
        } else if s_lower.contains("image/png") {
            Some(ContentType::Png)
        } else if s_lower.contains("image/jpeg") || s_lower.contains("image/jpg") {
            Some(ContentType::Jpeg)
        } else if s_lower.contains("image/gif") {
            Some(ContentType::Gif)
        } else if s_lower.contains("image/svg+xml") {
            Some(ContentType::Svg)
        } else if s_lower.contains("application/pdf") {
            Some(ContentType::Pdf)
        } else if s_lower.contains("application/zip") {
            Some(ContentType::Zip)
        } else if s_lower.contains("application/octet-stream") {
            Some(ContentType::Binary)
        } else {
            None
        }
    }

    /// Returns the file extension commonly associated with this content type.
    /// 
    /// # Returns
    /// 
    /// A string slice containing the file extension (without the dot).
    /// 
    /// # Examples
    /// 
    /// ```
    /// use roverfy::http::ContentType;
    /// 
    /// assert_eq!(ContentType::Html.extension(), "html");
    /// assert_eq!(ContentType::Json.extension(), "json");
    /// assert_eq!(ContentType::Png.extension(), "png");
    /// ```
    pub fn extension(&self) -> &str {
        match self {
            ContentType::Html => "html",
            ContentType::Json => "json",
            ContentType::PlainText => "txt",
            ContentType::Xml => "xml",
            ContentType::FormUrlEncoded => "form",
            ContentType::Css => "css",
            ContentType::JavaScript => "js",
            ContentType::Png => "png",
            ContentType::Jpeg => "jpg",
            ContentType::Gif => "gif",
            ContentType::Svg => "svg",
            ContentType::Pdf => "pdf",
            ContentType::Zip => "zip",
            ContentType::Binary => "bin",
        }
    }

    /// Checks if this content type is text-based.
    /// 
    /// Text-based content types can be safely treated as UTF-8 encoded strings.
    /// 
    /// # Returns
    /// 
    /// `true` if the content type is text-based, `false` otherwise.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use roverfy::http::ContentType;
    /// 
    /// assert!(ContentType::Html.is_text());
    /// assert!(ContentType::Json.is_text());
    /// assert!(!ContentType::Png.is_text());
    /// ```
    pub fn is_text(&self) -> bool {
        matches!(
            self,
            ContentType::Html | ContentType::Json | ContentType::PlainText | 
            ContentType::Xml | ContentType::FormUrlEncoded | ContentType::Css | 
            ContentType::JavaScript
        )
    }

    /// Checks if this content type is an image.
    /// 
    /// # Returns
    /// 
    /// `true` if the content type is an image, `false` otherwise.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use roverfy::http::ContentType;
    /// 
    /// assert!(ContentType::Png.is_image());
    /// assert!(ContentType::Jpeg.is_image());
    /// assert!(!ContentType::Html.is_image());
    /// ```
    pub fn is_image(&self) -> bool {
        matches!(
            self,
            ContentType::Png | ContentType::Jpeg | ContentType::Gif | ContentType::Svg
        )
    }
}

impl fmt::Display for ContentType {
    /// Formats the content type as a string.
    /// 
    /// This is equivalent to calling `as_str()`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for ContentType {
    type Err = ContentTypeParseError;

    /// Attempts to parse a content type from a string.
    /// 
    /// This implementation provides better error handling than the `from_str` method
    /// by returning a specific error type when parsing fails.
    /// 
    /// # Errors
    /// 
    /// Returns `ContentTypeParseError` if the string cannot be parsed as a valid content type.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use roverfy::http::ContentType;
    /// use std::str::FromStr;
    /// 
    /// assert!(ContentType::from_str("application/json").is_ok());
    /// assert!(ContentType::from_str("invalid/type").is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ContentType::parse_str(s).ok_or(ContentTypeParseError {
            input: s.to_string(),
        })
    }
}