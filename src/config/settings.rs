use once_cell::sync::OnceCell;
use std::sync::Mutex;
use std::fmt::Debug;
use std::path::{Path, PathBuf};

/// Trait defining the necessary settings for the application.
///
/// This allows for custom settings implementations. The trait must be `Debug`, `Send`, `Sync`,
/// and `'static` to be stored in a global static context.
pub trait Settings: Debug + Send + Sync + 'static {
    fn get_templates_path(&self) -> String;
    fn get_statics_path(&self) -> &String;
}

/// The default settings implementation.
#[derive(Debug)]
pub struct BaseSettings {
    pub templates_path: String,
    pub static_path: String,
    pub base_path: String,
    pub url_config: Option<String>,
}

// The global configuration, storing a boxed trait object to allow for different
// settings implementations.
static GLOBAL_CONFIG: OnceCell<Mutex<Box<dyn Settings>>> = OnceCell::new();

/// Finds the project root by searching upwards from the executable's location
/// for a `Cargo.toml` file. This makes the path independent of the
/// current working directory.
fn find_project_root() -> PathBuf {
    let mut current_dir = std::env::current_exe()
        .expect("Failed to get executable path")
        .parent()
        .expect("Failed to get executable directory")
        .to_path_buf();

    loop {
        if current_dir.join("Cargo.toml").exists() {
            return current_dir;
        }

        if !current_dir.pop() {
            // Fallback to current working directory if root is reached.
            return std::env::current_dir().expect("Failed to get current directory");
        }
    }
}

impl BaseSettings {
    /// Creates a default `BaseSettings` instance.
    /// The `base_path` is automatically determined by finding the project root.
    pub fn default() -> Self {
        Self {
            templates_path: "templates".to_string(),
            static_path: "static".to_string(),
            base_path: find_project_root().to_string_lossy().to_string(),
            url_config: None,
        }
    }
    
    pub fn from_file(
        _file_path: &str,
    ) -> Result<Self, String> {
        // Here you would implement logic to read from a file and parse the settings.
        // For simplicity, we return default settings.
        Ok(Self::default())
    }

    fn resolve_path(&self, path: &str) -> String {
        Path::new(&self.base_path)
            .join(path)
            .to_string_lossy()
            .to_string()
    }
}

impl Settings for BaseSettings {
    fn get_templates_path(&self) -> String {
        self.resolve_path(&self.templates_path)
    }

    fn get_statics_path(&self) -> &String {
        &self.static_path
    }
}

/// Initializes the global configuration with a given settings implementation.
///
/// This function should be called once at the start of the application.
/// It will panic if called more than once.
pub fn init_config<S: Settings>(settings: S) {
    GLOBAL_CONFIG
        .set(Mutex::new(Box::new(settings)))
        .expect("Failed to initialize global config. It may have already been initialized.");
}

/// Retrieves a lock on the global configuration.
///
/// This allows reading the settings from anywhere in the application.
/// It will panic if the configuration has not been initialized.
pub fn get_config() -> std::sync::MutexGuard<'static, Box<dyn Settings>> {
    GLOBAL_CONFIG
        .get()
        .expect("Global config not initialized. Call init_config first.")
        .lock()
        .expect("Failed to lock global config.")
}

/// Initializes a default configuration for testing purposes.
///
/// This function is intended for use in documentation tests and unit tests.
/// It uses `get_or_init` to be safe to call multiple times, initializing
/// the global config only if it hasn't been already.
pub fn init_test_config() {
    GLOBAL_CONFIG.get_or_init(|| Mutex::new(Box::new(BaseSettings::default())));
}
