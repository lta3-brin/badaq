use anyhow::Result;
use polars::prelude::*;
use std::io::Cursor;
use tokio::net::TcpStream;

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

    pub fn calc_data(&self, lf: LazyFrame, other: LazyFrame) -> Result<LazyFrame> {
        let nlf = lf
            .with_row_index("index", None)
            .join(
                other.with_row_index("index", None),
                [col("index")],
                [col("index")],
                JoinArgs::new(JoinType::Inner),
            )
            .select([
                (col("kol1") - col("kol1_right")).alias("kl1"),
                (col("kol1_mean") - col("kol1_mean_right")).alias("kl1_mean"),
                (col("kol1_std") - col("kol1_std_right")).alias("kl1_std"),
                (col("kol1_rms") - col("kol1_rms_right")).alias("kl1_rms"),
                (col("kol2") - col("kol2_right")).alias("kl2"),
                (col("kol2_mean") - col("kol2_mean_right")).alias("kl2_mean"),
                (col("kol2_std") - col("kol2_std_right")).alias("kl2_std"),
                (col("kol2_rms") - col("kol2_rms_right")).alias("kl2_rms"),
                (col("kol3") - col("kol3_right")).alias("kl3"),
                (col("kol3_mean") - col("kol3_mean_right")).alias("kl3_mean"),
                (col("kol3_std") - col("kol3_std_right")).alias("kl3_std"),
                (col("kol3_rms") - col("kol3_rms_right")).alias("kl3_rms"),
                (col("kol4") - col("kol4_right")).alias("kl4"),
                (col("kol4_mean") - col("kol4_mean_right")).alias("kl4_mean"),
                (col("kol4_std") - col("kol4_std_right")).alias("kl4_std"),
                (col("kol4_rms") - col("kol4_rms_right")).alias("kl4_rms"),
                (col("kol5") - col("kol5_right")).alias("kl5"),
                (col("kol5_mean") - col("kol5_mean_right")).alias("kl5_mean"),
                (col("kol5_std") - col("kol5_std_right")).alias("kl5_std"),
                (col("kol5_rms") - col("kol5_rms_right")).alias("kl5_rms"),
                (col("kol6") - col("kol6_right")).alias("kl6"),
                (col("kol6_mean") - col("kol6_mean_right")).alias("kl6_mean"),
                (col("kol6_std") - col("kol6_std_right")).alias("kl6_std"),
                (col("kol6_rms") - col("kol6_rms_right")).alias("kl6_rms"),
            ])
            .select([
                (1481.48482578329 * col("kl1")
                    + 2474.88899479221 * col("kl2")
                    + 49.1840375906808 * col("kl3")
                    + -0.0511660757922492 * col("kl4")
                    + -0.115345955315131 * col("kl5")
                    + 0.00422783876641308 * col("kl6"))
                .alias("k1"),
                (1481.48482578329 * col("kl1_mean")
                    + 2474.88899479221 * col("kl2_mean")
                    + 49.1840375906808 * col("kl3_mean")
                    + -0.0511660757922492 * col("kl4_mean")
                    + -0.115345955315131 * col("kl5_mean")
                    + 0.00422783876641308 * col("kl6_mean"))
                .alias("k1_mean"),
                (1481.48482578329 * col("kl1_rms")
                    + 2474.88899479221 * col("kl2_rms")
                    + 49.1840375906808 * col("kl3_rms")
                    + -0.0511660757922492 * col("kl4_rms")
                    + -0.115345955315131 * col("kl5_rms")
                    + 0.00422783876641308 * col("kl6_rms"))
                .alias("k1_rms"),
                col("kl1_std").alias("k1_std"),
            ]);

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

        Ok(nlf)
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
