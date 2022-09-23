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
        let mut lbl = vec![];

        for (idx, line) in dt.trim().split(" ").enumerate() {
            let line = line.trim().parse::<f32>()?;
            let a = line - app_state.koreksi[idx];

            lbl.push(a);
        }

        let k1 = 1481.48482578329 * lbl[0] + 2474.88899479221 * lbl[1] + 49.1840375906808 * lbl[2] - 0.0511660757922492 * lbl[3] - 0.115345955315131 * lbl[4] + 0.00422783876641308 * lbl[5];
        let k2 = 16.4352534976702 * lbl[0] + 26.9975537262353 * lbl[1] - 0.021689888082859 * lbl[2] + 3.29520163582838 * lbl[3] + 9.87647639575807 * lbl[4] + 0.0452086818621841 * lbl[5];
        let k3 = 700.96720273151 * lbl[0] - 593.068144510914 * lbl[1] - 11.7924782744019 * lbl[2] - 1.33147879149042 * lbl[3] - 3.96182250409738 * lbl[4] - 0.0123280913871845 * lbl[5];
        let k4 = -90.3481683057332 * lbl[0] - 111.881088810194 * lbl[1] - 1.83966205205915 * lbl[2] - 0.030205770481682 * lbl[3] - 0.0296322284753154 * lbl[4] + 1.00985294186794 * lbl[5];
        let k5 = -4.3158405735096 * lbl[0] + 4.19570408253097 * lbl[1] - 0.175423910741572 * lbl[2] + 1.26471001928057 * lbl[3] - 3.80397612831038 * lbl[4] - 0.0080627365900235 * lbl[5];
        let k6 = 0.686460938560072 * lbl[0] + 1029.31912478392 * lbl[1] - 20.6554471572674 * lbl[2] + 0.00420047631039713 * lbl[3] - 0.00443423766009805 * lbl[4] - 0.463697212185334 * lbl[5];

        ws.write_message(
            Message::Text(
                format!("{k1},{k2},{k3},{k4},{k5},{k6}")
            )
        ).unwrap();
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
