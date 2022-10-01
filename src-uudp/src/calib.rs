use std::{path::Path, sync::{Mutex, Arc}};

use crate::{err::AppErr, model::AppState};

pub fn init(st: &Arc<Mutex<AppState>>) -> Result<(), AppErr> {
    let filename = "calib.csv";
    let is_exist = Path::new(filename).exists();
    let mut app_state = st.lock().unwrap();

    if !is_exist {
        let mut wtr = csv::Writer::from_path(filename)?;

        wtr.write_record(&["1481.48482578329", "2474.88899479221", "49.1840375906808", "-0.0511660757922492", "-0.115345955315131", "0.00422783876641308"])?;
        wtr.write_record(&["16.4352534976702", "26.9975537262353", "-0.021689888082859", "3.29520163582838", "9.87647639575807", "0.0452086818621841"])?;
        wtr.write_record(&["700.96720273151", "-593.068144510914", "-11.7924782744019", "-1.33147879149042", "-3.96182250409738", "-0.0123280913871845"])?;
        wtr.write_record(&["-90.3481683057332", "-111.881088810194", "-1.83966205205915", "-0.030205770481682", "-0.0296322284753154", "1.00985294186794"])?;
        wtr.write_record(&["-4.3158405735096", "4.19570408253097", "-0.175423910741572", "1.26471001928057", "-3.80397612831038", "-0.0080627365900235"])?;
        wtr.write_record(&["0.686460938560072", "1029.31912478392", "-20.6554471572674", "0.00420047631039713", "-0.00443423766009805", "-0.463697212185334"])?;
        wtr.flush()?;
    }

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(filename)?;

    let mut coll: Vec<String> = vec![];
    for result in rdr.records() {
        let record = result?;

        let a = &record[0];
        let b = &record[1];
        let c = &record[2];
        let d = &record[3];
        let e = &record[4];
        let f = &record[5];

        let dt = format!("{},{},{},{},{},{}", a, b, c, d, e, f);

        coll.push(dt)
    }

    app_state.koef = coll;

    Ok(())
}