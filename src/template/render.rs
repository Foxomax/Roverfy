use crate::http::response::{ResponseBuilder, HttpResponseBuilder};
use crate::http::StatusCode;
use std::collections::HashMap;
use crate::http::contenttypes::ContentType;
use crate::template::TEMPLATE_ENGINE;
use tera::Context;
use tera::Error;

pub fn render(
    template_path: String,
    context: &HashMap<String, String>,
) -> Result<Vec<u8>, Error> {
    let ct = ContentType::Html;
    let context = Context::from_serialize(context)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    let renderer = TEMPLATE_ENGINE.render(template_path.as_str(), &context)?;

    let response = HttpResponseBuilder.build(
        StatusCode::OK,
        vec![("Content-Type".to_string(), ct.as_str().to_string())],
        renderer,
    );

    Ok(response)
}