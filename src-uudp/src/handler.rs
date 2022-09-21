use std::cell::RefCell;
use std::net::TcpStream;
use tungstenite::{Message, WebSocket};

use crate::err::AppErr;

pub fn parse_message(ws: &mut WebSocket<TcpStream>, msg: String) -> Result<(), AppErr> {
    let msg = msg.trim().to_string();
    let mut f = "".to_string();
    let mut coor: Vec<f32> = vec![];

    get_name(msg.clone(), &mut f);

    if msg.contains("CORR1") {
        let dt = get_corr(msg.clone())?.trim().to_string();

        for dd in dt.split(" ") {
            let d = dd.parse::<f32>()?;

            coor.push(d);
        }
    }

    println!("{}", f);

    if msg.contains("DSN") {
        let dt = calc_corr(msg.clone())?;
        let lbl = "aaz".to_string();

        for (_idx, _line) in dt.split(" ").enumerate() {
            // println!("{}:{}", idx, line);
            // let line = line.trim().parse::<f32>()?;
            // let a = line - coor[idx];

            // lbl.push_str(format!("{},", a).as_str());
        }

        ws.write_message(Message::Text(lbl)).unwrap();
    }

    if msg.contains("ENDSEQ") {
        let lbl = "ENDSEQ".to_string();

        ws.write_message(Message::Text(lbl)).unwrap();
    }

    Ok(())
}

fn get_name(msg: String, f: &mut String) {
    if msg.contains("EXP") {
        let lines = msg.split("\n");

        for line in lines {
            if line.contains("EXP") {
                let ln: Vec<&str> = line.split(",").collect();

                f.push_str(&format!("{}-{}", ln[1].trim(), ln[2].trim()))
            }

            if line.contains("RUN") {
                let ln: Vec<&str> = line.split(",").collect();

                f.push_str(&format!("-RUN{}.csv", ln[1].trim()))
            }
        }

        // f.push_str(format!("{f}.csv").as_str());
    }
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
