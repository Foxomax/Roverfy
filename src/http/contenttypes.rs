pub enum ContentType {
    Html,
    Json,
    PlainText,
    Xml,
    FormUrlEncoded,
}

impl ContentType {
    pub fn as_str(&self) -> &str {
        match self {
            ContentType::Html => "text/html; charset=utf-8",
            ContentType::Json => "application/json; charset=utf-8",
            ContentType::PlainText => "text/plain; charset=utf-8",
            ContentType::Xml => "application/xml; charset=utf-8",
            ContentType::FormUrlEncoded => "application/x-www-form-urlencoded; charset=utf-8",
        }
    }
}