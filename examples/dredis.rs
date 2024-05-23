use std::{io::ErrorKind, net::SocketAddr};

use anyhow::Result;
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};
use tracing::{error, info, warn};

const MAX_BUFFER_SIZE: usize = 4096;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let addr = "0.0.0.0:6379";
    let listener = TcpListener::bind(addr).await?;
    info!("Listening on: {}", addr);
    loop {
        let (stream, s_addr) = listener.accept().await?;
        info!("Accepted connection from: {}", s_addr);
        tokio::spawn(async move {
            if let Err(e) = process_redis_stream(stream, s_addr).await {
                warn!("Failed to process connection with {}: {:?}", s_addr, e);
            }
        });
    }
}

async fn process_redis_stream(mut stream: TcpStream, addr: SocketAddr) -> Result<()> {
    loop {
        stream.readable().await?;
        let mut buf = Vec::with_capacity(MAX_BUFFER_SIZE);
        match stream.try_read_buf(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                info!("Read {} bytes", n);
                let s = String::from_utf8_lossy(&buf);
                info!("Received: {:?}", s);
                stream.write_all(b"+OK\r\n").await?;
            }
            Err(e) if e.kind() == ErrorKind::WouldBlock => {
                info!("Would block");
                continue;
            }
            Err(e) => {
                error!("Failed to read from socket; err = {:?}", e);
                return Err(e.into());
            }
        }
    }
    info!("Connection with {} closed", addr);
    Ok(())
}
