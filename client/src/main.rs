use std::env;

use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> io::Result<()> {
    let server_addr = env::var("TAURI_UPLOAD_SERVER").expect("could not find TAURI_UPLOAD_SERVER");
    let mut stream = TcpStream::connect(server_addr).await?;
    let mut file = File::open("").await?;

    let filename = "file.txt";
    let filename_size = filename.len() as u16;

    // Send the filename size and filename
    stream.write_all(&filename_size.to_be_bytes()).await?;
    stream.write_all(filename.as_bytes()).await?;

    let mut buf = [0; 1024];

    // Read file data and send it
    while let Ok(n) = file.read(&mut buf).await {
        if n == 0 {
            break;
        }
        stream.write_all(&buf[0..n]).await?;
    }

    Ok(())
}
