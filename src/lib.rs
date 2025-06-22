mod server;
mod http;
mod config;
mod template;
mod router;

pub enum Command {
    RunServer,
    PrintHelp,
    Version,
    Unknown,
}


pub struct Roverfy {
    args: Vec<String>,
}


// Library exports.
pub use crate::config::BaseSettings;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

trait CLI {
    fn run_server(&self) -> Result<(), String>;
    fn help(&self) -> Result<(), String>;
    fn version(&self) -> Result<(), String>;
}

impl Roverfy {
    pub fn new(args: Vec<String>, settings: BaseSettings) -> Self {
        settings.init_config();
        Roverfy { args }
    }

    fn parse_command(&self) -> Command {
        match self.args.get(1).map(|s| s.as_str()) {
            Some("serve") => Command::RunServer,
            Some("help") => Command::PrintHelp,
            Some("version") => Command::Version,
            _ => Command::Unknown,
        }
    }

    pub fn run(&self) -> Result<(), String> {
        match self.parse_command() {
            Command::RunServer => self.run_server(),
            Command::PrintHelp => self.help(),
            Command::Version => self.version(),
            Command::Unknown => {
                Err("Command not recognized. Use 'help' to see available commands.".to_string())
            }
        }
    }
}

impl CLI for Roverfy {
    fn run_server(&self) -> Result<(), String> {
        let server = server::Server::new("127.0.0.1".to_string(), 7878);
        server.start();
        Ok(())
    }
    
    fn help(&self) -> Result<(), String> {
        println!("Available commands:\n - serve: Starts the development server.\n - help: Displays this help message.\n - version: Displays the current version of Roverfy.");
        Ok(())
    }

    fn version(&self) -> Result<(), String> {
        println!("Roverfy version: {}", VERSION);
        Ok(())
    }
}
