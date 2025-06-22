use once_cell::sync::Lazy;
use tera::{Tera, Context};
use crate::config::{get_config, Settings};

pub struct TemplateEngine {
    tera: Tera,
}

impl TemplateEngine {
    pub fn new() -> Self {
        let dir = format!("{}/**/*", get_config().get_templates_path());
        let tera = Tera::new(&dir).unwrap();
        Self { tera }
    }

    pub fn render(&self, path: &str, ctx: &Context) -> Result<String, tera::Error> {
        self.tera.render(path, ctx)
    }
}

pub static TEMPLATE_ENGINE: Lazy<TemplateEngine> = Lazy::new(|| TemplateEngine::new());