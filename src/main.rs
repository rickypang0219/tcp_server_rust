use rayon::prelude::*;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

async fn handle_client(mut stream: TcpStream, data: Arc<Vec<u8>>) {
    let mut buffer = vec![0; 1024];
    match stream.read(&mut buffer).await {
        Ok(n) if n > 0 => {
            let request = String::from_utf8_lossy(&buffer[..n]);
            println!("Received request: {}", request);

            // Perform some parallel processing using rayon
            let processed_data: Vec<u8> = data.par_iter().map(|&byte| byte + 1).collect();

            if let Err(e) = stream.write_all(&processed_data).await {
                eprintln!("Failed to write response: {}", e);
            }
        }
        Ok(_) => {
            println!("Received empty request");
        }
        Err(e) => {
            eprintln!("Failed to read from client: {}", e);
        }
    }
}
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to Bind");
    println!("Server listening on 127.0.0.1:8080");

    // Create some data to process
    let data = Arc::new((0..255).collect::<Vec<u8>>());

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                let data = data.clone();
                tokio::spawn(async move {
                    handle_client(stream, data).await;
                });
            }
            Err(e) => {
                eprintln!("Failed to establish connection: {}", e);
            }
        }
    }
}
