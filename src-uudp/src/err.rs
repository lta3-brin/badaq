use std::{io, num::ParseFloatError, string};
use csv;

#[derive(Debug)]
pub enum AppErr {
    BindSocket(io::Error),
    BytesConversion(string::FromUtf8Error),
    FloatConversion(ParseFloatError),
    CsvError(csv::Error)
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

impl From<ParseFloatError> for AppErr {
    fn from(err: ParseFloatError) -> Self {
        AppErr::FloatConversion(err)
    }
}

impl From<csv::Error> for AppErr {
    fn from(err: csv::Error) -> Self {
        AppErr::CsvError(err)
    }
}
