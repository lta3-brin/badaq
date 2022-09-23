#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod err;
mod handler;

use err::AppErr;

// use crate::handler::udp_comm;

fn main() -> Result<(), AppErr> {
    // udp_comm()?;

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
