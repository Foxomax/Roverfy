mod server;
mod http;
mod config;
mod template;

pub struct Roverfy {
    args: Vec<String>,
}


// Library exports.
pub use crate::config::BaseSettings;

trait CLI {
    fn run_server(&self) -> Result<(), String>;
}

impl Roverfy {
    pub fn new(args: Vec<String>, settings: BaseSettings) -> Self {
        settings.init_config();
        Roverfy { args }
    }
}

impl CLI for Roverfy {
    fn run_server(&self) -> Result<(), String> {
        let server = server::Server::new("127.0.0.1".to_string(), 7878);
        server.start();
        Ok(())
    }
}
