use std::env;
use roverfy::Roverfy;
use roverfy::BaseSettings;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let settings = BaseSettings::default();
    let app = Roverfy::new(args, settings);
    if let Err(e) = app.run() {
        eprintln!("Error: {}", e);
    }
}
