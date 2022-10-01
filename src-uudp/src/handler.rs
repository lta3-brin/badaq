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

        let mut val: Vec<f32> = vec![];
        for koef in app_state.koef.to_owned() {
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

        let lbl = format!("{},{},{},{},{},{},{},{},{},{}", tmp[0], tmp[1], tmp[2], tmp[3], val[0], val[1], val[2], val[3], val[4], val[5]);

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
