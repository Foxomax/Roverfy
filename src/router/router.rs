

pub struct PathRouter {
    url: String,
    handler: Box<dyn Fn() -> String + Send + Sync>,
    name: Option<String>,
}

pub fn path(url: &str, handler: impl Fn() -> String + Send + Sync + 'static) -> PathRouter {
    PathRouter {
        url: url.to_string(),
        handler: Box::new(handler),
        name: None,
    }
}