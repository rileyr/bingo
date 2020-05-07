use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct HeaderReadError {}

impl Error for HeaderReadError {
    fn description(&self) -> &str {
        "failed to read header"
    }
    fn cause(&self) -> Option<&(dyn Error)> {
        None
    }
}

impl fmt::Display for HeaderReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "failed to read header")
    }
}
