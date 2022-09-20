use std::{io, string};

#[derive(Debug)]
pub enum AppErr {
    BindSocket(io::Error),
    BytesConversion(string::FromUtf8Error),
}

impl From<io::Error> for AppErr {
    fn from(err: io::Error) -> Self {
        AppErr::BindSocket(err)
    }
}

impl From<string::FromUtf8Error> for AppErr {
    fn from(err: string::FromUtf8Error) -> Self {
        AppErr::BytesConversion(err)
    }
}
