use std::fmt;
use std::error;

pub struct StringError {
    val: String,
}

impl StringError {
    pub fn new(val: &str) -> StringError {
        StringError{val: val.to_string()}
    }
}

impl error::Error for StringError {
    fn description(&self) -> &str {
        &self.val
    }
    fn cause(&self) -> Option<&(dyn error::Error)> {
        None
    }
}

impl fmt::Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl fmt::Debug for StringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}
