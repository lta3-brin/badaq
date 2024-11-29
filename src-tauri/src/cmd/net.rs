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
                    let mut buf = vec![0; 10000];
                    let state = app.state::<Mutex<AppState>>();
                    let mut state = state.lock().unwrap();

                    match stream.try_read(&mut buf) {
                        Ok(0) => {
                            onevent.send(format!("LOST")).unwrap();

                            break;
                        }

                        Ok(_) => match str::from_utf8(&buf) {
                            Ok(message) => {
                                if message.contains("EXP") {
                                    state.nama = klien.get_name(message.into());

                                    onevent.send(state.nama.clone()).unwrap();
                                } else if message.contains("CORR") {
                                    if message.contains("SEQ") {
                                        state.seq = klien.get_name(message.into());

                                        onevent.send(state.seq.clone()).unwrap();
                                    }

                                    match klien.parse_buff(message.trim().to_string()) {
                                        Ok(lf) => state.corr = lf,
                                        Err(err) => onevent
                                            .send(format!("ERROR:{}", err.to_string()))
                                            .unwrap(),
                                    }
                                } else if message.contains("SEQ") {
                                    state.seq = klien.get_name(message.into());

                                    onevent.send(state.seq.clone()).unwrap();
                                } else if message.contains("DSN") {
                                    let dsn = klien.get_name(message.into());

                                    match klien.parse_buff(message.trim().to_string()) {
                                        Ok(lf) => {
                                            let df = klien
                                                .calc_data(lf, state.corr.clone())
                                                .unwrap()
                                                .collect()
                                                .unwrap();

                                            let mut payload =
                                                format!("{},{},", state.seq.clone(), dsn);
                                            let koloms = vec![
                                                "k1", "k1_mean", "k1_rms", "k1_std", "k2",
                                                "k2_mean", "k2_rms", "k2_std", "k3", "k3_mean",
                                                "k3_rms", "k3_std", "k4", "k4_mean", "k4_rms",
                                                "k4_std", "k5", "k5_mean", "k5_rms", "k5_std",
                                                "k6", "k6_mean", "k6_rms", "k6_std",
                                            ];

                                            for kol in df.columns(koloms).unwrap() {
                                                if let Some(val) = kol.f64().unwrap().get(0) {
                                                    payload.push_str(&format!("{},", val))
                                                }
                                            }

                                            onevent.send(payload).unwrap();
                                        }

                                        Err(err) => onevent
                                            .send(format!("ERROR:{}", err.to_string()))
                                            .unwrap(),
                                    }
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
