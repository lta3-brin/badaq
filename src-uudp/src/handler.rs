use std::{
    net::TcpStream,
    sync::{Arc, Mutex},
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

    if msg.contains("EXP") {
        app_state.nama = get_name(msg.clone());
    }

    if msg.contains("CORR1") {
        let dt = get_corr(msg.clone())?.trim().to_string();

        for dd in dt.trim().split(" ") {
            let d = dd.parse::<f32>()?;

            app_state.koreksi.push(d);
        }
    }

    if msg.contains("SEQ") {
        let lines = msg.trim().to_string();
        let mut tmp: Vec<String> = vec![];

        for line in lines.split(",") {
            tmp.push(line.trim().to_string());
        }

        let lbl = format!("{},{}", tmp[0], tmp[1]);
        ws.write_message(Message::Text(lbl)).unwrap();
    }

    if msg.contains("DSN") {
        let dt = calc_corr(msg.clone())?;
        let mut lbl = "".to_string();

        for (idx, line) in dt.trim().split(" ").enumerate() {
            let line = line.trim().parse::<f32>()?;
            let a = line - app_state.koreksi[idx];

            lbl.push_str(format!("{a},").as_str());
        }

        ws.write_message(Message::Text(lbl)).unwrap();
    }

    if msg.contains("ENDRUN") {
        let lbl = "ENDRUN".to_string();

        ws.write_message(Message::Text(lbl)).unwrap();
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

fn get_corr(msg: String) -> Result<String, AppErr> {
    let mut lines = msg.trim().to_string();

    lines = corr_string(&mut lines, 1, false);

    Ok(lines)
}

fn calc_corr(msg: String) -> Result<String, AppErr> {
    let mut lines = msg.trim().to_string();

    if msg.contains("SEQ") {
        lines = corr_string(&mut lines, 4, true);
    } else {
        lines = corr_string(&mut lines, 2, false);
    }

    Ok(lines)
}

fn corr_string(lines: &mut String, start: usize, is_newline: bool) -> String {
    let mut tmp: Vec<String> = vec![];

    if is_newline {
        *lines = lines.replace("\n", "");
    }

    for line in lines.split(",") {
        tmp.push(line.to_string());
    }

    let lnn = &tmp[start..tmp.len() - 1].join(" ");

    format!("{}", lnn)
}
