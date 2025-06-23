/// Error type for content type parsing failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentTypeParseError {
    /// The input string that failed to parse
    pub input: String,
}

impl std::fmt::Display for ContentTypeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse content type from '{}'", self.input)
    }
}

impl std::error::Error for ContentTypeParseError {}
