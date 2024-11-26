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

                                    onevent.send("EXP,".into()).unwrap();
                                } else if message.contains("CORR1") {
                                    if let Err(err) = klien.corr_string(
                                        &mut state,
                                        message.trim().to_string(),
                                        false,
                                    ) {
                                        onevent.send(format!("ERROR:{}", err.to_string())).unwrap()
                                    }
                                } else if message.contains("DSN") {
                                    match klien.dsn_parsing(&mut state, message.trim().to_string())
                                    {
                                        Ok(lbl) => onevent.send(lbl).unwrap(),
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
