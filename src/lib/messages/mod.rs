pub mod header;
pub mod body;

pub struct Message {
    header: header::Header,
    body: body::Body,
}

pub fn new(header: header::Header, body_option: Option<body::Body>) -> Message {
    let body = match body_option {
        Some(b) => b,
        None => body::new(vec![0; body::DEFAULT_BODY_SIZE]),
    };

    Message{
        header: header,
        body: body,
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\n{}", self.header)
    }
}
