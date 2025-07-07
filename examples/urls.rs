use roverfy::{PathRouter, path};

let urls = vec![
    path("/", test, "test"),
    "http://localhost:8080",
    "https://api.example.com/v1/resource",
    "ftp://files.example.com/download",
];

fn test() {
    println!("HEllo");
}