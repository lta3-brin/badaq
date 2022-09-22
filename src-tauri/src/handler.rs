use tauri::api::process::Command;

use crate::err::AppErr;

pub fn udp_comm() -> Result<(), AppErr> {
    let tt = Command::new_sidecar("src-uudp")?;

    tt.spawn()?;

    Ok(())
}
