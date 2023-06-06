use anyhow::Result;
use named_pipe::PipeOptions;
use std::str;

const PIPE_NAME: &str = r"\\.\pipe\wnp_np";

fn main() -> Result<()> {
    let server = PipeOptions::new(PIPE_NAME).single()?;

    let mut server = server.wait()?;
    println!("client connected");

    let buf = vec![0; 1024];

    loop {
        let (n, x) = server.read_async_owned(buf.clone())?.wait()?;

        let y = x.unwrap();
        server = y.0;
        let msg = y.1;

        let msg = str::from_utf8(&msg[..n])?;
        println!("incoming message: {:?}", msg)
    }
}
