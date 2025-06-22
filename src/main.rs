use std::env;
use roverfy::Roverfy;
use roverfy::BaseSettings;

fn main() {
    let args: Vec<String> = env::args().collect();
    let settings = BaseSettings::default();
    Roverfy::new(args, settings);
}
