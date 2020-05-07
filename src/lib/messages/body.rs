pub struct Body {
    bytes: Vec<u8>,
}

pub const DEFAULT_BODY_SIZE: usize = 128;

pub fn new(bytes: Vec<u8>) -> Body {
    Body{bytes: bytes}
}
