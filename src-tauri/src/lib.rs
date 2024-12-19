mod cmd;

use std::sync::Mutex;

use tauri::Manager;

use aerolib::aerotauri::{deploy, env, state::AppState};
use cmd::net::try_connect;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout),
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::LogDir {
                        file_name: Some("logs".to_string()),
                    }),
                ])
                .max_file_size(100_000 /* bytes */)
                .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
                .level(log::LevelFilter::Info)
                .build(),
        )
        .setup(|app| {
            env::load_env(app)?;
            deploy::deploy_surreal(app)?;

            app.manage(Mutex::new(AppState::default()));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![try_connect])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
