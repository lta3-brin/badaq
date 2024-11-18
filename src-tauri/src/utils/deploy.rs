use anyhow::Result;
use colored::Colorize;
use std::sync::{Arc, Mutex};
use tauri::{App, Manager};
use tauri_plugin_shell::ShellExt;

pub fn deploy_surreal(app: &mut App) -> Result<()> {
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
}
