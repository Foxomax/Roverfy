use std::env;
use std::path::PathBuf;

pub trait Settings {
    fn get_template_path(&self) -> &PathBuf;
    fn get_static_path(&self) -> &PathBuf;
}

pub struct BaseSettings {
    pub template_path: PathBuf,
    pub static_path:   PathBuf,
}

impl BaseSettings {
    pub fn new() -> Self {
        let default_tpl = PathBuf::from("./templates");
        let default_sta = PathBuf::from("./static");

        let template_path = env::var_os("TEMPLATE_PATH")
            .map(PathBuf::from)
            .unwrap_or(default_tpl);

        let static_path = env::var_os("STATIC_PATH")
            .map(PathBuf::from)
            .unwrap_or(default_sta);

        BaseSettings { template_path, static_path }
    }
}

impl Settings for BaseSettings {
    fn get_template_path(&self) -> &PathBuf {
        &self.template_path
    }

    fn get_static_path(&self) -> &PathBuf {
        &self.static_path
    }
}


