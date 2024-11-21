use anyhow::Result;
use tokio::net::TcpStream;

#[derive(Debug)]
pub struct TcpKlien {
    addr: String,
}

impl TcpKlien {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub async fn get_stream(self) -> Result<TcpStream> {
        let stream = TcpStream::connect(self.addr).await?;

        Ok(stream)
    }
}
