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
        mut lines: String,
        is_newline: bool,
    ) -> String {
        let mut tmp: Vec<String> = vec![];

        if is_newline {
            lines = lines.replace("\n", "");
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

    pub fn corr_parsing(&self, state: &mut MutexGuard<AppState>, lines: String) -> Result<()> {
        let mut tmp = vec![];

        let text = self.corr_string(state, lines, false).trim().to_string();

        for dd in text.trim().split(",") {
            tmp.push(dd)
        }

        tmp.pop();

        for dd in tmp {
            let d = dd.parse::<f32>()?;
            state.koreksi.push(d);
        }

        Ok(())
    }

    pub fn dsn_parsing(&self, state: &mut MutexGuard<AppState>, lines: String) -> Result<String> {
        let msg;
        let mut tmp = vec![];

        if lines.contains("SEQ") {
            msg = self.corr_string(state, lines, true);

            for line in msg.trim().split(",") {
                tmp.push(line.trim());
            }
        } else {
            let lbl = state.seq.to_owned();
            msg = format!("{},{}", lbl, self.corr_string(state, lines, false));

            for line in msg.trim().split(",") {
                tmp.push(line.trim());
            }
        }

        tmp.pop();

        let r1 = tmp[4].trim().parse::<f32>()? - state.koreksi[0];
        let r2 = tmp[5].trim().parse::<f32>()? - state.koreksi[1];
        let r3 = tmp[6].trim().parse::<f32>()? - state.koreksi[2];
        let r4 = tmp[7].trim().parse::<f32>()? - state.koreksi[3];
        let r5 = tmp[8].trim().parse::<f32>()? - state.koreksi[4];
        let r6 = tmp[9].trim().parse::<f32>()? - state.koreksi[5];

        let mut val: Vec<f32> = vec![];
        for koef in state.koef.to_owned() {
            let k = koef.split(",").collect::<Vec<&str>>();
            let a = k[0].parse::<f32>()?;
            let b = k[1].parse::<f32>()?;
            let c = k[2].parse::<f32>()?;
            let d = k[3].parse::<f32>()?;
            let e = k[4].parse::<f32>()?;
            let f = k[5].parse::<f32>()?;

            let res = a * r1 + b * r2 + c * r3 + d * r4 + e * r5 + f * r6;

            val.push(res);
        }

        let lbl = format!(
            "{},{},{},{},{},{},{},{},{},{}",
            tmp[0], tmp[1], tmp[2], tmp[3], val[0], val[1], val[2], val[3], val[4], val[5]
        );

        state.koleksi.push(lbl.clone());

        Ok(lbl)
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
