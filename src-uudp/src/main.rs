mod err;

use std::net::TcpListener;
use std::net::UdpSocket;
use std::thread::spawn;
use tungstenite::{accept, Message};

use err::AppErr;

fn main() -> Result<(), AppErr> {
    let server = TcpListener::bind("127.0.0.1:9001")?;
    let socket = UdpSocket::bind("127.0.0.1:17845")?;

    println!("Running server...");

    for stream in server.incoming() {
        let socket = socket.try_clone()?;

        spawn(move || {
            let mut buf = [0; 1024];
            let mut websocket = accept(stream.unwrap()).unwrap();

            loop {
                socket.recv_from(&mut buf).unwrap();

                let msg = String::from_utf8(buf.to_vec()).unwrap();
                websocket.write_message(Message::Text(msg)).unwrap();

                // Clear the buffer
                for el in buf.iter_mut() {
                    *el = 0
                }
            }
        });
    }

    Ok(())
}
