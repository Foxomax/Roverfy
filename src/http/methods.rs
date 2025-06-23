/// Represents HTTP methods used in web requests.
/// 
/// This enum provides type-safe handling of HTTP methods with conversion
/// methods between string representations and enum variants.
/// 
/// # Examples
/// 
/// ```
/// use roverfy::http::Method;
/// 
/// let method = Method::from_str("POST").unwrap();
/// assert_eq!(method, Method::POST);
/// assert_eq!(method.as_str(), "POST");
/// 
/// let method = Method::from_str("get").unwrap();
/// assert_eq!(method, Method::GET);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Method {
    /// HTTP GET method - used for retrieving data
    GET,
    /// HTTP POST method - used for creating new resources
    POST,
    /// HTTP PUT method - used for updating entire resources
    PUT,
    /// HTTP DELETE method - used for removing resources
    DELETE,
    /// HTTP PATCH method - used for partial updates
    PATCH,
    /// HTTP HEAD method - used for retrieving headers only
    HEAD,
    /// HTTP OPTIONS method - used for CORS preflight requests
    OPTIONS,
}

impl Method {
    /// Converts a string to a Method enum variant.
    /// 
    /// This method performs case-insensitive matching, so "get", "GET", and "Get"
    /// all return `Some(Method::GET)`.
    /// 
    /// # Arguments
    /// 
    /// * `s` - The HTTP method string to convert
    /// 
    /// # Returns
    /// 
    /// `Some(Method)` if the string represents a valid HTTP method, `None` otherwise
    /// 
    /// # Examples
    /// 
    /// ```
    /// use roverfy::http::Method;
    /// 
    /// assert_eq!(Method::from_str("GET"), Some(Method::GET));
    /// assert_eq!(Method::from_str("post"), Some(Method::POST));
    /// assert_eq!(Method::from_str("INVALID"), None);
    /// ```
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "GET" => Some(Method::GET),
            "POST" => Some(Method::POST),
            "PUT" => Some(Method::PUT),
            "DELETE" => Some(Method::DELETE),
            "PATCH" => Some(Method::PATCH),
            "HEAD" => Some(Method::HEAD),
            "OPTIONS" => Some(Method::OPTIONS),
            _ => None,
        }
    }
    
    /// Returns the string representation of the HTTP method.
    /// 
    /// # Returns
    /// 
    /// A static string slice containing the HTTP method name
    /// 
    /// # Examples
    /// 
    /// ```
    /// use roverfy::http::Method;
    /// 
    /// assert_eq!(Method::GET.as_str(), "GET");
    /// assert_eq!(Method::POST.as_str(), "POST");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::DELETE => "DELETE",
            Method::PATCH => "PATCH",
            Method::HEAD => "HEAD",
            Method::OPTIONS => "OPTIONS",
        }
    }

    /// Checks if this HTTP method is safe according to RFC 7231.
    /// 
    /// Safe methods are those that don't have side effects and are
    /// primarily used for retrieving data.
    /// 
    /// # Returns
    /// 
    /// `true` if the method is safe, `false` otherwise
    /// 
    /// # Examples
    /// 
    /// ```
    /// use roverfy::http::Method;
    /// 
    /// assert!(Method::GET.is_safe());
    /// assert!(Method::HEAD.is_safe());
    /// assert!(!Method::POST.is_safe());
    /// assert!(!Method::DELETE.is_safe());
    /// ```
    pub fn is_safe(&self) -> bool {
        matches!(self, Method::GET | Method::HEAD | Method::OPTIONS)
    }

    /// Checks if this HTTP method is idempotent according to RFC 7231.
    /// 
    /// Idempotent methods can be called multiple times without changing
    /// the result beyond the initial application.
    /// 
    /// # Returns
    /// 
    /// `true` if the method is idempotent, `false` otherwise
    /// 
    /// # Examples
    /// 
    /// ```
    /// use roverfy::http::Method;
    /// 
    /// assert!(Method::GET.is_idempotent());
    /// assert!(Method::PUT.is_idempotent());
    /// assert!(Method::DELETE.is_idempotent());
    /// assert!(!Method::POST.is_idempotent());
    /// ```
    pub fn is_idempotent(&self) -> bool {
        matches!(self, Method::GET | Method::HEAD | Method::OPTIONS | Method::PUT | Method::DELETE)
    }

    /// Checks if this HTTP method can have a request body.
    /// 
    /// # Returns
    /// 
    /// `true` if the method can have a body, `false` otherwise
    /// 
    /// # Examples
    /// 
    /// ```
    /// use roverfy::http::Method;
    /// 
    /// assert!(Method::POST.can_have_body());
    /// assert!(Method::PUT.can_have_body());
    /// assert!(!Method::GET.can_have_body());
    /// ```
    pub fn can_have_body(&self) -> bool {
        matches!(self, Method::POST | Method::PUT | Method::PATCH)
    }

    /// Checks if this HTTP method is cacheable.
    /// 
    /// # Returns
    /// 
    /// `true` if the method is cacheable, `false` otherwise
    /// 
    /// # Examples
    /// 
    /// ```
    /// use roverfy::http::Method;
    /// 
    /// assert!(Method::GET.is_cacheable());
    /// assert!(Method::HEAD.is_cacheable());
    /// assert!(!Method::POST.is_cacheable());
    /// ```
    pub fn is_cacheable(&self) -> bool {
        matches!(self, Method::GET | Method::HEAD)
    }
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for Method {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Method::from_str(s).ok_or_else(|| format!("Invalid HTTP method: {}", s))
    }
}