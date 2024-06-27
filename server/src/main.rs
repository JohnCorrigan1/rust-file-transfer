use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:6969").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];
            let elapsed = std::time::Instant::now();
            // Read the filename size and then filename
            if let Ok(_) = socket.read_exact(&mut buf[0..2]).await {
                let size = u16::from_be_bytes([buf[0], buf[1]]) as usize;
                if let Ok(_) = socket.read_exact(&mut buf[0..size]).await {
                    let filename = String::from_utf8_lossy(&buf[0..size]).to_string();

                    // Open the file for writing
                    let filename = format!("received/{}", filename);
                    let mut file = tokio::fs::File::create(filename).await.unwrap();

                    // Read the file data from socket and write to file
                    while let Ok(n) = socket.read(&mut buf).await {
                        if n == 0 {
                            break;
                        }
                        file.write_all(&buf[0..n]).await.unwrap();
                    }
                }
            }
            println!("File received");
            println!("Elapsed time: {:?}", elapsed.elapsed().as_secs_f64());
        });
    }
}
