// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use colored::Colorize;
use std::sync::{Arc, Mutex};
use tauri::{path::BaseDirectory, Manager};
use tauri_plugin_shell::ShellExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let envpth = app.path().resolve(".env", BaseDirectory::Resource)?;
            dotenv::from_path(envpth)?;

            let pwd = std::env::var("SURREAL_PWD")?;
            let surreal_cmd = app.shell().sidecar("surreal")?.args([
                "start",
                "--user",
                "root",
                "--pass",
                &pwd,
                "rocksdb:badaq.db",
            ]);

            log::info!("{}", "Spawning database...".bright_magenta());
            let (_rx, comm) = surreal_cmd.spawn()?;
            let cmd = Arc::new(Mutex::new(Some(comm)));
            let cmdclone = cmd.clone();

            if let Some(window) = app.get_webview_window("main") {
                window.on_window_event(move |event| match event {
                    tauri::WindowEvent::Destroyed => match cmdclone.lock() {
                        Ok(mut cmdlock) => {
                            if let Some(cmdclone_lock) = cmdlock.take() {
                                if let Err(err) = cmdclone_lock.kill() {
                                    log::error!("{}", err.to_string().bright_red());
                                }
                            }
                        }

                        Err(err) => log::error!("{}", err.to_string().bright_red()),
                    },

                    _ => {}
                });
            }

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout),
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Folder {
                        path: std::path::PathBuf::from("./logs"),
                        file_name: None,
                    }),
                ])
                .max_file_size(100_000 /* bytes */)
                .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
                .level(log::LevelFilter::Info)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
