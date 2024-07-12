use std::{env, fs::File, io, path::Path};
// use tokio::fs::File;
use std::io::{BufReader, Read, Write};
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
// use tokio::net::TcpStream;
use std::net::TcpStream;

// const BUF_SIZE: usize = 4 * 1024 * 1024;
const BUF_SIZE: usize = 4 * 1024;
// const FOUR_MB: usize = 4 * 1024 * 1024;
// const EIGHT_MB: usize = 8 * 1024 * 1024;

#[tokio::main]
async fn main() -> io::Result<()> {
    let server_addr = env::var("FILE_UPLOAD_SERVER").expect("could not find FILE_UPLOAD_SERVER");
    let mut count = 0;
    let mut total_time = 0.0;
    // let results: Vec<String> = Vec::new();
    loop {
        let start = std::time::Instant::now();
        count += 1;

        let mut stream = TcpStream::connect(&server_addr)?;
        stream.set_nodelay(true)?;
        println!("Connected to server");
        let path = Path::new("/Users/johncorrigan/Downloads/hello.txt");
        let mut file = File::open(path)?;

        let filename = format!("std_hello_vec_{}mb_{}.txt", (BUF_SIZE / 1024 / 1024), count);

        let filename_size = filename.len() as u16;
        stream.write_all(&filename_size.to_be_bytes())?;
        stream.write_all(filename.trim().as_bytes())?;
        let file_type = 4u16;
        stream.write_all(&file_type.to_be_bytes())?;

        // let mut buf = vec![0; BUF_SIZE];
        // let mut buf: [u8; 1024 * 4] = [0; 1024 * 4];
        // let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
        // panic!("buf size: {}", buf.len());
        let mut reader = BufReader::with_capacity(1024 * 1024 * 16, file);
        // let mut reader = BufReader::new(file);
        // let mut new_file = File::create("new.txt")?;
        // std::io::copy(&mut reader, &mut new_file)?;
        // let mut
        std::io::copy(&mut reader, &mut stream)?;
        // reader.
        // std::io::copy(reader, )
        // while let Ok(n) = reader.read(&mut buf) {
        // std::io::copy*
        // if n == 0 {
        // break;
        // }
        // std::io::copy(&mut &buf[0..n], &mut stream)?;
        // std::io::copy(&mut file, &mut stream)?;
        // stream.write_all(&mut buf)?;
        // }
        // reader.read_line
        // while let Ok(n) = file.read(&mut buf).await {
        // if n == 0 {
        // break;
        // }
        // stream.write_all(&buf[0..n]).await?;
        // }

        total_time += start.elapsed().as_secs_f64();
        println!("File sent");
        println!("File {} took: {}", count, start.elapsed().as_secs_f64());
        if count == 5 {
            // println!("Total time for 5 files: {}", total_time);
            // println!("Average time over 5 files {}", total_time / 5.0);

            break;
        }
    }

    let mut file = File::create("results.txt")?;
    file.write(
        format!(
            "\nBufType: {}\n\nBufSize: {}kb\nTotal time for 5 files: {}\nAverage {}\n",
            "Vec",
            (BUF_SIZE / 1024),
            total_time,
            total_time / 5.0
        )
        .as_bytes(),
    )?;

    Ok(())
}
