use anyhow::Result;
use std::sync::MutexGuard;
use tokio::net::TcpStream;

use super::state::AppState;

#[derive(Debug)]
pub struct TcpKlien {
    addr: String,
}

impl TcpKlien {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub async fn get_stream(&self) -> Result<TcpStream> {
        let stream = TcpStream::connect(self.addr.clone()).await?;

        Ok(stream)
    }

    pub fn get_name(&self, msg: String) -> String {
        let lines = msg.split("\n");

        let mut f = "".to_string();
        for line in lines {
            if line.contains("EXP") {
                let ln: Vec<&str> = line.split(",").collect();

                f.push_str(&format!("{}-{}", ln[1].trim(), ln[2].trim()))
            }

            if line.contains("RUN") {
                let ln: Vec<&str> = line.split(",").collect();

                f.push_str(&format!("-RUN{}", ln[1].trim()))
            }
        }

        format!("{f}.csv")
    }

    pub fn corr_string(
        &self,
        state: &mut MutexGuard<AppState>,
        lines: &mut String,
        is_newline: bool,
    ) -> String {
        let mut tmp: Vec<String> = vec![];

        if is_newline {
            *lines = lines.replace("\n", "");
        }

        for line in lines.split(",") {
            tmp.push(line.trim().to_string());
        }

        if tmp[0] == "SEQ" {
            state.seq = format!("{},{}", tmp[0], tmp[1]);
        } else if tmp[0] == "CORR1" {
            tmp.remove(0);
        }

        tmp.join(",")
    }

    pub fn save_csv(&self, filename: &String, koleksi: Vec<String>) -> Result<()> {
        let mut writer = csv::Writer::from_path(filename)?;

        writer.write_record(&[
            "sec", "secnum", "dsn", "dsnnum", "k1", "k2", "k3", "k4", "k5", "k6",
        ])?;

        for dt in koleksi {
            writer.write_record(dt.split(","))?;
        }

        writer.flush()?;

        Ok(())
    }
}
