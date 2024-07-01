use std::{env, io};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

// const ONE_MB: usize = 1024 * 1024;
// const FOUR_MB: usize = 4 * 1024 * 1024;
const EIGHT_MB: usize = 8 * 1024 * 1024;

#[tokio::main]
async fn main() -> io::Result<()> {
    let server_addr = env::var("FILE_UPLOAD_SERVER").expect("could not find FILE_UPLOAD_SERVER");
    let mut count = 0;
    let mut total_time = 0.0;
    loop {
        let start = std::time::Instant::now();
        count += 1;

        let mut stream = TcpStream::connect(&server_addr).await?;
        stream.set_nodelay(true)?;
        println!("Connected to server");

        let mut file = File::open("file.txt").await?;

        let filename = "file.txt";

        let filename_size = filename.len() as u16;
        stream.write_all(&filename_size.to_be_bytes()).await?;
        stream.write_all(filename.trim().as_bytes()).await?;

        let mut buf = vec![0; EIGHT_MB];

        while let Ok(n) = file.read(&mut buf).await {
            if n == 0 {
                break;
            }
            stream.write_all(&buf[0..n]).await?;
        }

        total_time += start.elapsed().as_secs_f64();
        println!("File sent");
        println!("File {} took: {}", count, start.elapsed().as_secs_f64());
        if count == 5 {
            println!("Total time for 5 files: {}", total_time);
            println!("Average time over 5 files {}", total_time / 5.0);
            break;
        } else {
            println!("Waiting 5 seconds before sending next file");
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    }
    Ok(())
}
