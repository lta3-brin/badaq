use core::str;
use std::io;

use anyhow::Result;
use tokio::net::TcpStream;

pub async fn try_connect(addr: &str) -> Result<()> {
    let stream = TcpStream::connect(addr).await?;

    loop {
        stream.readable().await?;
        let mut buf = vec![0; 1024];

        match stream.try_read(&mut buf) {
            Ok(0) => break,
            Ok(_) => {
                println!("{}", str::from_utf8(&buf)?);
            }

            Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }

            Err(err) => log::error!("{}", err.to_string()),
        }
    }

    Ok(())
}
