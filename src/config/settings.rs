use std::fmt::format;
use once_cell::sync::OnceCell;
use std::sync::Mutex;

pub trait Settings {
    fn get_templates_path(&self) -> String;
    fn get_static_path(&self) -> &String;
}

#[derive(Debug)]
pub struct BaseSettings {
    pub templates_path: String,
    pub static_path: String,
    pub base_path: String,
}

static GLOBAL_CONFIG: OnceCell<Mutex<BaseSettings>> = OnceCell::new();

impl BaseSettings {
    pub fn default() -> Self {
        Self {
            templates_path: "templates".to_string(),
            static_path: "static".to_string(),
            base_path: std::env::current_dir()
                .expect("Failed to get current directory")
                .to_string_lossy()
                .to_string(),
        }
    }

    pub fn init_config(self) {
        GLOBAL_CONFIG
            .set(Mutex::new(self))
            .expect("Failed to initialize global config");
    }
}

impl Settings for BaseSettings {
    fn get_templates_path(&self) -> String {
        format!("{}/{}", self.base_path, self.templates_path)
    }

    fn get_static_path(&self) -> &String {
        &self.static_path
    }
}

pub fn get_config() -> std::sync::MutexGuard<'static, BaseSettings> {
    GLOBAL_CONFIG
        .get()
        .expect("Global config not initialized")
        .lock()
        .expect("Failed to lock global config")
}
