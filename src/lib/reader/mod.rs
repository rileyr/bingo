
use std::error::Error;
use crate::lib::messages;
use crate::lib::errs;

use tokio::prelude::*;
use tokio::io::{ReadHalf};
use tokio::net::{TcpStream};

pub struct Reader {
    stream: ReadHalf<TcpStream>,
    hdr_buf: Vec<u8>,
}

impl Reader {
    pub fn new(stream: ReadHalf<TcpStream>) -> Reader {
        Reader{
            stream: stream,
            hdr_buf: vec![0; messages::header::HEADER_LEN],
        }
    }

    pub async fn next_message(&mut self) -> Result<messages::Message, Box<dyn Error>> {
        let h = self.next_header().await?;
        Ok(messages::new(h, None))
    }

    async fn next_header(&mut self) -> Result<messages::header::Header, Box<dyn Error>> {
        let n = self.stream.read(&mut self.hdr_buf).await?;

        if n < messages::header::HEADER_LEN {
            return Err(Box::new(errs::StringError::new("unexpected header size")));
        }

        let (h, b, _) = unsafe { self.hdr_buf.align_to::<messages::header::Header>() };
        if !h.is_empty() {
            return Err(Box::new(errs::StringError::new("failed to parse header")));
        }

        return Ok(b[0])
    }
}
