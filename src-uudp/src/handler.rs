use std::{fmt::format, net::TcpStream};

use tungstenite::{Message, WebSocket};

use crate::err::AppErr;

pub fn parse_message(ws: &mut WebSocket<TcpStream>, msg: String) -> Result<(), AppErr> {
    let msg = msg.trim().to_string();
    let mut f = "".to_string();
    let mut coor: Vec<f32> = vec![];

    if msg.contains("EXP") {
        f.push_str(get_name(msg.clone()).as_str());
    }

    if msg.contains("CORR1") {
        coor = get_corr(msg.clone())?;
    }

    if msg.contains("DSN") {
        calc_corr(msg.clone())?;
    }

    if msg.contains("ENDSEQ") {
        let lbl = "ENDSEQ".to_string();

        ws.write_message(Message::Text(lbl)).unwrap();
    }

    ws.write_message(Message::Text(f)).unwrap();

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

fn get_corr(msg: String) -> Result<Vec<f32>, AppErr> {
    let lines = msg.trim().split(",");
    let mut coors: Vec<f32> = vec![];

    for (idx, line) in lines.enumerate() {
        if idx != 0 && idx != 7 {
            let line = line.trim().parse::<f32>()?;

            coors.push(line);
        }
    }

    Ok(coors)
}

fn calc_corr(msg: String) -> Result<Vec<f32>, AppErr> {
    let mut lines = msg.trim().to_string();

    if msg.contains("SEQ") {
        let mut tmp: Vec<String> = vec![];
        let lns = lines.replace("\n", "");

        for line in lns.split(",") {
            let line = line.trim();

            tmp.push(line.trim().to_string())
        }

        let lnn = &tmp[4..tmp.len() - 1].join(" ");

        lines = format!("{}", lnn);
    } else {
        let mut tmp: Vec<String> = vec![];

        for line in lines.split(",") {
            tmp.push(line.to_string());
        }

        let lnn = &tmp[2..tmp.len() - 1].join(" ");

        lines = format!("{}", lnn);
    }

    Ok(vec![])
}
