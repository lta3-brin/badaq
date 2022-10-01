use csv;
use std::{
    net::TcpStream,
    sync::{Arc, Mutex, MutexGuard},
};
use tungstenite::{Message, WebSocket};

use crate::{err::AppErr, model::AppState};

pub fn parse_message(
    ws: &mut WebSocket<TcpStream>,
    msg: String,
    st: &Arc<Mutex<AppState>>,
) -> Result<(), AppErr> {
    let msg = msg.trim().to_string();
    let mut app_state = st.lock().unwrap();

    if msg.contains("Connection Test") {
        let lbl = "connected,".to_string();

        ws.write_message(Message::Text(lbl)).unwrap();
    }

    if msg.contains("EXP") {
        app_state.nama = get_name(msg.clone());
        let lbl = "EXP,".to_string();

        ws.write_message(Message::Text(lbl)).unwrap();
    }

    if msg.contains("CORR1") {
        let mut lines = msg.clone().trim().to_string();
        let mut tmp = vec![];

        lines = corr_string(&mut app_state, &mut lines, false).trim().to_string();

        for dd in lines.trim().split(",") {
            tmp.push(dd)
        }

        tmp.pop();

        for dd in tmp {
            let d = dd.parse::<f32>()?;
            app_state.koreksi.push(d);
        }
    }

    if msg.contains("DSN") {
        let mut lines = msg.clone().trim().to_string();
        let mut tmp = vec![];

        if msg.contains("SEQ") {
            lines = corr_string(&mut app_state, &mut lines, true);

            for line in lines.trim().split(",") {
                tmp.push(line.trim());
            }
        } else {
            let lbl = &app_state.seq.to_owned();

            lines = format!("{},{}", lbl, corr_string(&mut app_state, &mut lines, false));

            for line in lines.trim().split(",") {
                tmp.push(line.trim());
            }
        }

        tmp.pop();

        let r1 = tmp[4].trim().parse::<f32>()? - app_state.koreksi[0];
        let r2 = tmp[5].trim().parse::<f32>()? - app_state.koreksi[1];
        let r3 = tmp[6].trim().parse::<f32>()? - app_state.koreksi[2];
        let r4 = tmp[7].trim().parse::<f32>()? - app_state.koreksi[3];
        let r5 = tmp[8].trim().parse::<f32>()? - app_state.koreksi[4];
        let r6 = tmp[9].trim().parse::<f32>()? - app_state.koreksi[5];

        let k1 = 1481.48482578329 * r1 + 2474.88899479221 * r2 + 49.1840375906808 * r3 - 0.0511660757922492 * r4 - 0.115345955315131 * r5 + 0.00422783876641308 * r6;
        let k2 = 16.4352534976702 * r1 + 26.9975537262353 * r2 - 0.021689888082859 * r3 + 3.29520163582838 * r4 + 9.87647639575807 * r5 + 0.0452086818621841 * r6;
        let k3 = 700.96720273151 * r1 - 593.068144510914 * r2 - 11.7924782744019 * r3 - 1.33147879149042 * r4 - 3.96182250409738 * r5 - 0.0123280913871845 * r6;
        let k4 = -90.3481683057332 * r1 - 111.881088810194 * r2 - 1.83966205205915 * r3 - 0.030205770481682 * r4 - 0.0296322284753154 * r5 + 1.00985294186794 * r6;
        let k5 = -4.3158405735096 * r1 + 4.19570408253097 * r2 - 0.175423910741572 * r3 + 1.26471001928057 * r4 - 3.80397612831038 * r5 - 0.0080627365900235 * r6;
        let k6 = 0.686460938560072 * r1 + 1029.31912478392 * r2 - 20.6554471572674 * r3 + 0.00420047631039713 * r4 - 0.00443423766009805 * r5 - 0.463697212185334 * r6;

        let lbl = format!("{},{},{},{},{},{},{},{},{},{}", tmp[0], tmp[1], tmp[2], tmp[3], k1, k2, k3, k4, k5, k6);

        app_state.koleksi.push(lbl.clone());
        ws.write_message(
            Message::Text(lbl)
        ).unwrap();
    }

    if msg.contains("ENDSEQ,") {
        let lbl = "ENDSEQ".to_string();

        ws.write_message(Message::Text(lbl)).unwrap();
    }

    if msg.contains("ENDRUN") {
        let lbl = "ENDRUN".to_string();

        ws.write_message(Message::Text(lbl)).unwrap();
        save_csv(&app_state.nama, app_state.koleksi.to_owned())?;

        app_state.nama = "".to_string();
        app_state.koreksi = vec![];
        app_state.koleksi = vec![];
    }

    Ok(())
}

fn get_name(msg: String) -> String {
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

fn corr_string(st: &mut MutexGuard<AppState>, lines: &mut String, is_newline: bool) -> String {
    let mut tmp: Vec<String> = vec![];

    if is_newline {
        *lines = lines.replace("\n", "");
    }

    for line in lines.split(",") {
        tmp.push(line.trim().to_string());
    }

    if tmp[0] == "SEQ" {
        st.seq = format!("{},{}", tmp[0], tmp[1]);
    } else if tmp[0] == "CORR1" {
        tmp.remove(0);
    }

    tmp.join(",")
}

fn save_csv(filename: &String, koleksi: Vec<String>) -> Result<(), AppErr> {
    let mut writer = csv::Writer::from_path(filename)?;

    writer.write_record(&["sec", "secnum", "dsn", "dsnnum", "k1", "k2", "k3", "k4", "k5", "k6"])?;

    for dt in koleksi {
        writer.write_record(dt.split(","))?;
    }

    writer.flush()?;

    Ok(())
}
