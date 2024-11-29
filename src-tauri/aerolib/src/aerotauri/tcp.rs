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
        let mut ln;
        let mut f = "".to_string();

        for line in msg.lines() {
            if line.contains("EXP") {
                ln = line.split(",").collect::<Vec<&str>>();

                f.push_str(&format!("{}-{}", ln[1].trim(), ln[2].trim()))
            } else if line.contains("RUN") {
                ln = line.split(",").collect::<Vec<&str>>();

                f.push_str(&format!("-RUN{}", ln[1].trim()))
            } else if line.contains("SEQ") {
                ln = line.split(",").collect::<Vec<&str>>();

                f.push_str(&format!("{}-{}", ln[0].trim(), ln[1].trim()))
            } else if line.contains("DSN") {
                ln = line.split(",").collect::<Vec<&str>>();

                f.push_str(&format!("{}-{}", ln[0].trim(), ln[1].trim()))
            }
        }

        f
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
        // Per baris
        // Aslinya: wtr.write_record(&["1481.48482578329", "2474.88899479221", "49.1840375906808", "-0.0511660757922492", "-0.115345955315131", "0.00422783876641308"])?;
        let a1 = 1481.48482578329;
        let a2 = 2474.88899479221;
        let a3 = 49.1840375906808;
        let a4 = -0.0511660757922492;
        let a5 = -0.115345955315131;
        let a6 = 0.00422783876641308;

        let b1 = 16.4352534976702;
        let b2 = 26.9975537262353;
        let b3 = -0.021689888082859;
        let b4 = 3.29520163582838;
        let b5 = 9.87647639575807;
        let b6 = 0.0452086818621841;

        let c1 = 700.96720273151;
        let c2 = -593.068144510914;
        let c3 = -11.7924782744019;
        let c4 = -1.33147879149042;
        let c5 = -3.96182250409738;
        let c6 = -0.0123280913871845;

        let d1 = -90.3481683057332;
        let d2 = -111.881088810194;
        let d3 = -1.83966205205915;
        let d4 = -0.030205770481682;
        let d5 = -0.0296322284753154;
        let d6 = 1.00985294186794;

        let e1 = -4.3158405735096;
        let e2 = 4.19570408253097;
        let e3 = -0.175423910741572;
        let e4 = 1.26471001928057;
        let e5 = -3.80397612831038;
        let e6 = -0.0080627365900235;

        let f1 = 0.686460938560072;
        let f2 = 1029.31912478392;
        let f3 = -20.6554471572674;
        let f4 = 0.00420047631039713;
        let f5 = -0.00443423766009805;
        let f6 = -0.463697212185334;

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
                (lit(a1) * col("kl1")
                    + lit(a2) * col("kl2")
                    + lit(a3) * col("kl3")
                    + lit(a4) * col("kl4")
                    + lit(a5) * col("kl5")
                    + lit(a6) * col("kl6"))
                .alias("k1"),
                (lit(a1) * col("kl1_mean")
                    + lit(a2) * col("kl2_mean")
                    + lit(a3) * col("kl3_mean")
                    + lit(a4) * col("kl4_mean")
                    + lit(a5) * col("kl5_mean")
                    + lit(a6) * col("kl6_mean"))
                .alias("k1_mean"),
                (lit(a1) * col("kl1_rms")
                    + lit(a2) * col("kl2_rms")
                    + lit(a3) * col("kl3_rms")
                    + lit(a4) * col("kl4_rms")
                    + lit(a5) * col("kl5_rms")
                    + lit(a6) * col("kl6_rms"))
                .alias("k1_rms"),
                col("kl1_std").alias("k1_std"),
                (lit(b1) * col("kl1")
                    + lit(b2) * col("kl2")
                    + lit(b3) * col("kl3")
                    + lit(b4) * col("kl4")
                    + lit(b5) * col("kl5")
                    + lit(b6) * col("kl6"))
                .alias("k2"),
                (lit(b1) * col("kl1_mean")
                    + lit(b2) * col("kl2_mean")
                    + lit(b3) * col("kl3_mean")
                    + lit(b4) * col("kl4_mean")
                    + lit(b5) * col("kl5_mean")
                    + lit(b6) * col("kl6_mean"))
                .alias("k2_mean"),
                (lit(b1) * col("kl1_rms")
                    + lit(b2) * col("kl2_rms")
                    + lit(b3) * col("kl3_rms")
                    + lit(b4) * col("kl4_rms")
                    + lit(b5) * col("kl5_rms")
                    + lit(b6) * col("kl6_rms"))
                .alias("k2_rms"),
                col("kl2_std").alias("k2_std"),
                (lit(c1) * col("kl1")
                    + lit(c2) * col("kl2")
                    + lit(c3) * col("kl3")
                    + lit(c4) * col("kl4")
                    + lit(c5) * col("kl5")
                    + lit(c6) * col("kl6"))
                .alias("k3"),
                (lit(c1) * col("kl1_mean")
                    + lit(c2) * col("kl2_mean")
                    + lit(c3) * col("kl3_mean")
                    + lit(c4) * col("kl4_mean")
                    + lit(c5) * col("kl5_mean")
                    + lit(c6) * col("kl6_mean"))
                .alias("k3_mean"),
                (lit(c1) * col("kl1_rms")
                    + lit(c2) * col("kl2_rms")
                    + lit(c3) * col("kl3_rms")
                    + lit(c4) * col("kl4_rms")
                    + lit(c5) * col("kl5_rms")
                    + lit(c6) * col("kl6_rms"))
                .alias("k3_rms"),
                col("kl3_std").alias("k3_std"),
                (lit(d1) * col("kl1")
                    + lit(d2) * col("kl2")
                    + lit(d3) * col("kl3")
                    + lit(d4) * col("kl4")
                    + lit(d5) * col("kl5")
                    + lit(d6) * col("kl6"))
                .alias("k4"),
                (lit(d1) * col("kl1_mean")
                    + lit(d2) * col("kl2_mean")
                    + lit(d3) * col("kl3_mean")
                    + lit(d4) * col("kl4_mean")
                    + lit(d5) * col("kl5_mean")
                    + lit(d6) * col("kl6_mean"))
                .alias("k4_mean"),
                (lit(d1) * col("kl1_rms")
                    + lit(d2) * col("kl2_rms")
                    + lit(d3) * col("kl3_rms")
                    + lit(d4) * col("kl4_rms")
                    + lit(d5) * col("kl5_rms")
                    + lit(d6) * col("kl6_rms"))
                .alias("k4_rms"),
                col("kl4_std").alias("k4_std"),
                (lit(e1) * col("kl1")
                    + lit(e2) * col("kl2")
                    + lit(e3) * col("kl3")
                    + lit(e4) * col("kl4")
                    + lit(e5) * col("kl5")
                    + lit(e6) * col("kl6"))
                .alias("k5"),
                (lit(e1) * col("kl1_mean")
                    + lit(e2) * col("kl2_mean")
                    + lit(e3) * col("kl3_mean")
                    + lit(e4) * col("kl4_mean")
                    + lit(e5) * col("kl5_mean")
                    + lit(e6) * col("kl6_mean"))
                .alias("k5_mean"),
                (lit(e1) * col("kl1_rms")
                    + lit(e2) * col("kl2_rms")
                    + lit(e3) * col("kl3_rms")
                    + lit(e4) * col("kl4_rms")
                    + lit(e5) * col("kl5_rms")
                    + lit(e6) * col("kl6_rms"))
                .alias("k5_rms"),
                col("kl5_std").alias("k5_std"),
                (lit(f1) * col("kl1")
                    + lit(f2) * col("kl2")
                    + lit(f3) * col("kl3")
                    + lit(f4) * col("kl4")
                    + lit(f5) * col("kl5")
                    + lit(f6) * col("kl6"))
                .alias("k6"),
                (lit(f1) * col("kl1_mean")
                    + lit(f2) * col("kl2_mean")
                    + lit(f3) * col("kl3_mean")
                    + lit(f4) * col("kl4_mean")
                    + lit(f5) * col("kl5_mean")
                    + lit(f6) * col("kl6_mean"))
                .alias("k6_mean"),
                (lit(f1) * col("kl1_rms")
                    + lit(f2) * col("kl2_rms")
                    + lit(f3) * col("kl3_rms")
                    + lit(f4) * col("kl4_rms")
                    + lit(f5) * col("kl5_rms")
                    + lit(f6) * col("kl6_rms"))
                .alias("k6_rms"),
                col("kl6_std").alias("k6_std"),
            ]);

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
