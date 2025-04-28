use crate::http::response::{ResponseBuilder, HttpResponseBuilder};
use crate::http::StatusCode;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use crate::config::{BaseSettings, Settings};

pub fn render(
    template_path: String,
    context: &HashMap<String, String>,
) -> Result<Vec<u8>, std::io::Error> {
    let template_path = BaseSettings::new().get_template_path().join(template_path);
    let mut file = File::open(template_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    for (key, value) in context {
        contents = contents.replace(&format!("{{{{ {} }}}}", key), value);
    }

    let response = HttpResponseBuilder.build(
        StatusCode::OK,
        vec![("Content-Type".to_string(), "text/html".to_string())],
        contents,
    );

    Ok(response)
}