use crate::http::response::{ResponseBuilder, HttpResponseBuilder};
use crate::http::StatusCode;
use std::collections::HashMap;
use crate::config::{get_config, Settings};
use crate::http::contenttypes::ContentType;
use crate::template::tera_renderer::{TeraRenderer};
use tera::Context;

pub fn render(
    template_path: String,
    context: &HashMap<String, String>,
) -> Result<Vec<u8>, std::io::Error> {
    let ct = ContentType::Html;
    let context = Context::from_serialize(context)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    let config = get_config();
    let render = TeraRenderer::new(
        format!("{}/{}", config.get_template_path(), template_path),
        context.clone(),
    );
    let output = render
        .render()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    let response = HttpResponseBuilder.build(
        StatusCode::OK,
        vec![("Content-Type".to_string(), ct.as_str().to_string())],
        output,
    );

    Ok(response)
}