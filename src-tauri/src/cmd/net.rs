use core::str;
use std::{io, sync::Mutex};
use tauri::{AppHandle, Manager};

use aerolib::aerotauri::{state::AppState, tcp::TcpKlien};

#[tauri::command]
pub async fn try_connect(app: AppHandle, addr: String, onevent: tauri::ipc::Channel<String>) {
    let klien = TcpKlien::new(addr);

    match klien.get_stream().await {
        Ok(stream) => loop {
            match stream.readable().await {
                Ok(_) => {
                    let mut buf = vec![0; 1024];

                    match stream.try_read(&mut buf) {
                        Ok(0) => {
                            onevent.send(format!("LOST")).unwrap();

                            break;
                        }

                        Ok(_) => match str::from_utf8(&buf) {
                            Ok(message) => {
                                let state = app.state::<Mutex<AppState>>();
                                let mut state = state.lock().unwrap();

                                if message.contains("EXP") {
                                    state.nama = klien.get_name(message.into());

                                    onevent.send("EXP,".into()).unwrap()
                                } else if message.contains("CORR1") {
                                    let mut lines = message.clone().trim().to_string();
                                    let mut tmp = vec![];

                                    lines = klien
                                        .corr_string(&mut state, &mut lines, false)
                                        .trim()
                                        .to_string();

                                    for dd in lines.trim().split(",") {
                                        tmp.push(dd)
                                    }

                                    tmp.pop();

                                    for dd in tmp {
                                        let d = dd.parse::<f32>()?;
                                        state.koreksi.push(d);
                                    }
                                } else if message.contains("DSN") {
                                    let mut lines = message.clone().trim().to_string();
                                    let mut tmp = vec![];

                                    if msg.contains("SEQ") {
                                        lines = klien.corr_string(&mut state, &mut lines, true);

                                        for line in lines.trim().split(",") {
                                            tmp.push(line.trim());
                                        }
                                    } else {
                                        let lbl = &state.seq.to_owned();

                                        lines = format!(
                                            "{},{}",
                                            lbl,
                                            corr_string(&mut state, &mut lines, false)
                                        );

                                        for line in lines.trim().split(",") {
                                            tmp.push(line.trim());
                                        }
                                    }

                                    tmp.pop();

                                    let r1 = tmp[4].trim().parse::<f32>()? - state.koreksi[0];
                                    let r2 = tmp[5].trim().parse::<f32>()? - state.koreksi[1];
                                    let r3 = tmp[6].trim().parse::<f32>()? - state.koreksi[2];
                                    let r4 = tmp[7].trim().parse::<f32>()? - state.koreksi[3];
                                    let r5 = tmp[8].trim().parse::<f32>()? - state.koreksi[4];
                                    let r6 = tmp[9].trim().parse::<f32>()? - state.koreksi[5];

                                    let mut val: Vec<f32> = vec![];
                                    for koef in state.koef.to_owned() {
                                        let k = koef.split(",").collect::<Vec<&str>>();
                                        let a = k[0].parse::<f32>()?;
                                        let b = k[1].parse::<f32>()?;
                                        let c = k[2].parse::<f32>()?;
                                        let d = k[3].parse::<f32>()?;
                                        let e = k[4].parse::<f32>()?;
                                        let f = k[5].parse::<f32>()?;

                                        let res =
                                            a * r1 + b * r2 + c * r3 + d * r4 + e * r5 + f * r6;

                                        val.push(res);
                                    }

                                    let lbl = format!(
                                        "{},{},{},{},{},{},{},{},{},{}",
                                        tmp[0],
                                        tmp[1],
                                        tmp[2],
                                        tmp[3],
                                        val[0],
                                        val[1],
                                        val[2],
                                        val[3],
                                        val[4],
                                        val[5]
                                    );

                                    state.koleksi.push(lbl.clone());

                                    onevent.send(lbl).unwrap()
                                } else if message.contains("ENDSEQ") {
                                    onevent.send("ENDSEQ".into()).unwrap()
                                } else if message.contains("ENDRUN") {
                                    onevent.send("ENDRUN".into()).unwrap()
                                }
                            }

                            Err(err) => onevent.send(format!("ERROR:{}", err.to_string())).unwrap(),
                        },

                        Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => {
                            continue;
                        }

                        Err(err) => onevent.send(format!("ERROR:{}", err.to_string())).unwrap(),
                    }
                }

                Err(err) => onevent.send(format!("ERROR:{}", err.to_string())).unwrap(),
            }
        },

        Err(err) => onevent.send(format!("ERROR:{}", err.to_string())).unwrap(),
    }
}
