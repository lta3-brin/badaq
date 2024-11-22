use anyhow::Result;
use tauri::{path::BaseDirectory, App, Manager};

pub fn load_env(app: &mut App) -> Result<()> {
    let envpth = app.path().resolve(".env", BaseDirectory::Resource)?;
    dotenv::from_path(envpth)?;

    Ok(())
}
