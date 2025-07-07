use std::future::Future;
use tokio::io;
use tokio::io::{AsyncRead, AsyncWrite};
use crate::http::HttpResponse;

trait View {
    
    fn get(&self, request: &str) -> HttpResponse;
    
    fn post(&self, request: &str) -> HttpResponse;
}