use anyhow::Result;
use named_pipe::PipeClient;
use std::{ str, thread, time::Duration };

const PIPE_NAME: &str = r"\\.\pipe\wnp_np";

fn main() -> Result<()> {
    let mut client = PipeClient::connect(PIPE_NAME)?;
    println!("{:?}", client);

    loop {
        let (_n, x) = client.write_async_owned(b"Ping!".to_vec())?.wait()?;

        let y = x.unwrap();
        client = y.0;

        thread::sleep(Duration::from_millis(1000));
    }
}
