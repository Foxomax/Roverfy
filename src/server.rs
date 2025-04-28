use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use std::collections::HashMap;
use crate::http::request::Request;
use crate::template::render;
use std::env;


pub struct Server {
    pub host: String,
    pub port: u16,
}

impl Server {
    pub fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }

    pub fn start(&self) {
        let listener = TcpListener::bind(format!("{}:{}", self.host, self.port)).unwrap();
        println!("Server listening on {}:{}", self.host, self.port);

        for stream in listener.incoming() {
            println!("Incoming connection");
            match stream {
                Ok(stream) => {
                    self.handle_client(stream).expect("TODO: panic message");
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                }
            }
        }
    }

    pub fn stop(&self) {
        println!("Server stopped");

    }

    fn handle_client(&self, mut stream: TcpStream) -> Result<(), std::io::Error> {
        let mut reader = BufReader::new(stream.try_clone()?);
        let mut buffer = String::new();

        while !buffer.contains("\r\n\r\n") {
            let mut temp_buffer = [0; 512];
            let bytes_read = reader.read(&mut temp_buffer)?;
            if bytes_read == 0 {
                break;
            }
            buffer.push_str(&String::from_utf8_lossy(&temp_buffer[..bytes_read]));
        }

        let request = Request::new(&buffer);
        println!("Request: {:?}", request);

        let ruta = env::current_dir().unwrap();
        println!("RootPath: {:?}", ruta);

        let response: Vec<u8> = render("test/index.html".to_string(), &HashMap::default())?;
        stream.write_all(&response)?;

        Ok(())
    }

    pub fn restart(&self) {
        println!("Server restarted");
        self.start();
    }
}