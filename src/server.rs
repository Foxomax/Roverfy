use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use std::collections::HashMap;
use crate::http::request::Request;
use crate::template::render;
use tera::Error;

pub struct Server {
    pub host: String,
    pub port: u16,
}

impl Server {
    pub fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }

    pub async fn start(&self) {
        let listener = TcpListener::bind(format!("{}:{}", self.host, self.port)).await.unwrap();
        loop {
            let (socket, _) = listener.accept().await.unwrap();
            tokio::spawn(async move {
                Server::process(socket).await.expect("Failed to process connection");
            });
        }
    }

    pub fn stop(&self) {
        println!("Server stopped");
    }

    async fn process(mut socket: TcpStream) -> Result<(), std::io::Error> {
        let mut reader = BufReader::new(&mut socket);
        let mut buffer = String::new();

        while !buffer.contains("\r\n\r\n") {
            let mut temp_buffer = [0; 512];
            let bytes_read = reader.read(&mut temp_buffer).await?;
            if bytes_read == 0 {
                break;
            }
            buffer.push_str(&String::from_utf8_lossy(&temp_buffer[..bytes_read]));
        }

        let _request = Request::new(&buffer);

        let mut context = HashMap::new();
        context.insert("hola".to_string(), "Mundo".to_string());

        let response: Result<Vec<u8>, Error> = render("test/index.html".to_string(), &context);
        let response = match response {
            Ok(res) => res,
            Err(e) => {
                eprintln!("Error rendering template: {}", e);
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Template rendering error"));
            }
        };
        socket.write_all(&response).await?;

        Ok(())
    }

    pub async fn restart(&self) {
        println!("Server restarted");
        self.start().await;
    }
}