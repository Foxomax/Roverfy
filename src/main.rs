mod server;
mod settings;
mod request;

fn main() {
    let server = server::Server::new("127.0.0.1".to_string(), 7878);
    server.start();
}
