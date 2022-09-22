use tauri::{api, Error};

#[derive(Debug)]
pub enum AppErr {
    TauriComm(Error),
    TauriApi(api::Error),
}

impl From<Error> for AppErr {
    fn from(err: Error) -> Self {
        AppErr::TauriComm(err)
    }
}

impl From<api::Error> for AppErr {
    fn from(err: api::Error) -> Self {
        AppErr::TauriApi(err)
    }
}
