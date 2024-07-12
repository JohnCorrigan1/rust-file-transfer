use std::{
    fs::File,
    io::{copy, BufReader, Read},
};
// use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use std::net::{TcpListener, TcpStream};

use tokio::time::error::Error;
// use tokio::net::{TcpListener, TcpStream};

const EIGHT_MB: usize = 8 * 1024 * 1024;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let listener = TcpListener::bind("0.0.0.0:6060").expect("should work");

    loop {
        let (mut socket, _) = listener.accept().expect("should accept");

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            let elapsed = std::time::Instant::now();

            println!("Received connection");
            // let mut reader = BufReader::new(&mut socket);
            let mut reader = BufReader::with_capacity(1024 * 1024 * 16, &mut socket);

            // reader.read_exact(2);
            // reader.read(&mut buf);
            // reader.read_exact
            if let Ok(_) = reader.read_exact(&mut buf[0..2]) {
                let size = u16::from_be_bytes([buf[0], buf[1]]) as usize;
                if let Ok(_) = reader.read_exact(&mut buf[0..size]) {
                    let filename = String::from_utf8_lossy(&buf[0..size]).to_string();
                    if let Ok(_) = reader.read_exact(&mut buf[0..2]) {
                        println!("{}:{}", buf[0], buf[1]);
                        // let file_type = FileType::from_u16(u16::from_be_bytes([buf[0], buf[1]]));
                        // println!("{:?}", file_type);
                        // let destination = file_type.file_destination(&mut socket).await;
                        let destination = "Documents";
                        let filename = format!("/mnt/sdb1/{}/{}", destination, filename);

                        std::fs::create_dir_all(format!("/mnt/sdb1/{}", destination)).unwrap();

                        //             let mut file = tokio::fs::File::create(filename).await.unwrap();
                        let mut file = File::create(filename).unwrap();
                        copy(&mut reader, &mut file).unwrap();

                        //             while let Ok(n) = socket.read(&mut buf).await {
                        //                 if n == 0 {
                        //                     break;
                        //                 }
                        //                 file.write_all(&buf[0..n]).await.unwrap();
                        // }
                    }
                }
            }
            println!("File received");
            println!("Elapsed time: {:?}", elapsed.elapsed().as_secs_f64());
        });
    }
}

enum FileType {
    Movie,
    Show,
    Image,
    Document,
}

impl FileType {
    fn from_u16(value: u16) -> Self {
        match value {
            1 => Self::Movie,
            2 => Self::Show,
            3 => Self::Image,
            4 => Self::Document,
            _ => panic!("Invalid file type"),
        }
    }

    async fn file_destination(&self, socket: &mut TcpStream) -> String {
        match self {
            Self::Movie => String::from("Movies"),
            Self::Show => {
                let mut buf = vec![0; 2048];
                socket.read_exact(&mut buf[0..2]).unwrap();
                let show_name_size = u16::from_be_bytes([buf[0], buf[1]]) as usize;

                socket.read_exact(&mut buf[0..show_name_size]).unwrap();

                let show_name = String::from_utf8_lossy(&buf[0..show_name_size]).to_string();
                format!("Shows/{}", show_name)
            }
            Self::Image => String::from("Images"),
            Self::Document => String::from("Documents"),
        }
    }
}
