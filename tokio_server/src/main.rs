use anyhow::Result;
use std::io;
use std::{str, thread, time::Duration};
use tokio::net::windows::named_pipe::ServerOptions;

const PIPE_NAME: &str = r"\\.\pipe\wnp_tokio";

#[tokio::main]
async fn main() -> Result<()> {
    let server = ServerOptions::new()
        .first_pipe_instance(true)
        .create(PIPE_NAME)?;
    println!("{server:?}");
    println!("{:?}", server.info()?);

    server.connect().await?;
    println!("{server:?}");

    let mut buf = vec![0; 1024];

    loop {
        server.readable().await?;

        match server.try_read(&mut buf) {
            Ok(n) => {
                let msg = str::from_utf8(&buf[..n])?;
                println!("incoming message: {:?}", msg)
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(100));
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }
}
