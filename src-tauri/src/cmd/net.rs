use core::str;
use std::io;

use aerolib::aerotauri::tcp::TcpKlien;

#[tauri::command]
pub async fn try_connect(addr: String, onevent: tauri::ipc::Channel<String>) {
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
                                onevent.send(message.to_string()).unwrap();
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
