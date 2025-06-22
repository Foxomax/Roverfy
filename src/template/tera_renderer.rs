use tera::{Tera, Context};

pub struct TeraRenderer {
    pub template_path: String,
    pub context: Context,
}

impl TeraRenderer {
    pub fn new(template_path: String, context: Context) -> Self {
        TeraRenderer { template_path, context }
    }

    pub fn render(&self) -> Result<String, tera::Error> {
        let tera = Tera::new(self.template_path.as_str())?;
        tera.render(&self.template_path, &self.context)
    }
}
