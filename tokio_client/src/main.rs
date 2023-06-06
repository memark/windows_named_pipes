use anyhow::Result;
use std::{io, thread, time::Duration};
use tokio::net::windows::named_pipe::ClientOptions;

const PIPE_NAME: &str = r"\\.\pipe\wnp_tokio";

#[tokio::main]
async fn main() -> Result<()> {
    let client = ClientOptions::new().open(PIPE_NAME)?;

    println!("{:?}", client);

    loop {
        client.writable().await?;

        match client.try_write(b"Ping!") {
            Ok(_) => {
                thread::sleep(Duration::from_millis(1000));
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
