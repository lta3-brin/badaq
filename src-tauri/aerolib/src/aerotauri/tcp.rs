use anyhow::Result;
use polars::prelude::*;
use std::{io::Cursor, sync::MutexGuard};
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

    pub fn parse_buff(&self, lines: String) -> Result<LazyFrame> {
        let csv_parsing = CsvParseOptions::default().with_truncate_ragged_lines(true);
        let lf = CsvReadOptions::default()
            .with_parse_options(csv_parsing)
            .with_skip_rows(6)
            .with_n_rows(Some(34))
            .with_ignore_errors(true)
            .with_has_header(false)
            .into_reader_with_file_handle(Cursor::new(lines.into_bytes()))
            .finish()?
            .lazy()
            .select([
                col("column_3").first().alias("kol1"),
                col("column_3").mean().alias("kol1_mean"),
                col("column_3").std(0).alias("kol1_std"),
                (col("column_3") * col("column_3"))
                    .mean()
                    .sqrt()
                    .alias("kol1_rms"),
                col("column_4").first().alias("kol2"),
                col("column_4").mean().alias("kol2_mean"),
                col("column_4").std(0).alias("kol2_std"),
                (col("column_4") * col("column_4"))
                    .mean()
                    .sqrt()
                    .alias("kol2_rms"),
                col("column_5").first().alias("kol3"),
                col("column_5").mean().alias("kol3_mean"),
                col("column_5").std(0).alias("kol3_std"),
                (col("column_5") * col("column_5"))
                    .mean()
                    .sqrt()
                    .alias("kol3_rms"),
                col("column_6").first().alias("kol4"),
                col("column_6").mean().alias("kol4_mean"),
                col("column_6").std(0).alias("kol4_std"),
                (col("column_6") * col("column_6"))
                    .mean()
                    .sqrt()
                    .alias("kol4_rms"),
                col("column_7").first().alias("kol5"),
                col("column_7").mean().alias("kol5_mean"),
                col("column_7").std(0).alias("kol5_std"),
                (col("column_7") * col("column_7"))
                    .mean()
                    .sqrt()
                    .alias("kol5_rms"),
                col("column_8").first().alias("kol6"),
                col("column_8").mean().alias("kol6_mean"),
                col("column_8").std(0).alias("kol6_std"),
                (col("column_8") * col("column_8"))
                    .mean()
                    .sqrt()
                    .alias("kol6_rms"),
            ]);

        Ok(lf)
    }

    pub fn dsn_parsing(&self, _state: &mut MutexGuard<AppState>, _lines: String) -> Result<String> {
        // let msg;
        // let mut tmp = vec![];

        // if lines.contains("SEQ") {
        //     msg = self.corr_string(state, lines, true);

        //     for line in msg.trim().split(",") {
        //         tmp.push(line.trim());
        //     }
        // } else {
        //     let lbl = state.seq.to_owned();
        //     msg = format!("{},{}", lbl, self.corr_string(state, lines, false).unwrap());

        //     for line in msg.trim().split(",") {
        //         tmp.push(line.trim());
        //     }
        // }

        // tmp.pop();

        // let r1 = tmp[4].trim().parse::<f32>()? - state.koreksi[0];
        // let r2 = tmp[5].trim().parse::<f32>()? - state.koreksi[1];
        // let r3 = tmp[6].trim().parse::<f32>()? - state.koreksi[2];
        // let r4 = tmp[7].trim().parse::<f32>()? - state.koreksi[3];
        // let r5 = tmp[8].trim().parse::<f32>()? - state.koreksi[4];
        // let r6 = tmp[9].trim().parse::<f32>()? - state.koreksi[5];

        // let mut val: Vec<f32> = vec![];
        // for koef in state.koef.to_owned() {
        //     let k = koef.split(",").collect::<Vec<&str>>();
        //     let a = k[0].parse::<f32>()?;
        //     let b = k[1].parse::<f32>()?;
        //     let c = k[2].parse::<f32>()?;
        //     let d = k[3].parse::<f32>()?;
        //     let e = k[4].parse::<f32>()?;
        //     let f = k[5].parse::<f32>()?;

        //     let res = a * r1 + b * r2 + c * r3 + d * r4 + e * r5 + f * r6;

        //     val.push(res);
        // }

        // let lbl = format!(
        //     "{},{},{},{},{},{},{},{},{},{}",
        //     tmp[0], tmp[1], tmp[2], tmp[3], val[0], val[1], val[2], val[3], val[4], val[5]
        // );

        // state.koleksi.push(lbl.clone());

        Ok("lbl".to_string())
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
