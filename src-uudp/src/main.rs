mod err;
mod handler;
mod model;

use std::net::TcpListener;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::thread::spawn;
use tungstenite::accept;

use crate::err::AppErr;
use crate::handler::parse_message;
use crate::model::AppState;

fn main() -> Result<(), AppErr> {
    let server = TcpListener::bind("127.0.0.1:9001")?;
    let socket = UdpSocket::bind("127.0.0.1:17845")?;

    println!("Running server...");

    let state = Arc::new(Mutex::new(AppState {
        nama: "".to_string(),
        koreksi: vec![],
    }));

    for stream in server.incoming() {
        let socket = socket.try_clone()?;
        let state = Arc::clone(&state);

        spawn(move || {
            let mut buf = [0; 1024];
            let mut websocket = accept(stream.unwrap()).unwrap();

            loop {
                socket.recv_from(&mut buf).unwrap();

                let msg = String::from_utf8(buf.to_vec()).unwrap();
                parse_message(&mut websocket, msg.clone(), &state).unwrap();

                // Clear the buffer
                for el in buf.iter_mut() {
                    *el = 0
                }
            }
        });
    }

    Ok(())
}
