use std::time::{SystemTime, UNIX_EPOCH};
use std::mem::size_of;
use std::slice;

pub const HEADER_LEN: usize = size_of::<Header>();

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Header {
    pub ts: u128,
    pub typ: u32,
    pub len: u32,
    pub seq: u64,
}

pub fn new(message_type: u32, seq_num: u64, message_len: Option<u32>) -> Header {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let in_ms = since_the_epoch.as_millis();
    let len = match message_len {
        Some(l) => l,
        None => 0,
    };

    Header{
        ts: in_ms,
        typ: message_type,
        len: len,
        seq: seq_num,
    }
}

impl Header {
    pub fn to_bytes(&self) -> &[u8] {
        let p: *const Header = self;
        let p: *const u8 = p as *const u8;
        let s: &[u8] = unsafe {
            slice::from_raw_parts(p, HEADER_LEN)
        };
        return s;
    }
}

impl std::fmt::Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\nheader: \n    typ: {}\n    len: {}\n    ts:  {}\n    seq: {}", self.typ, self.len, self.ts, self.seq)
    }
}
