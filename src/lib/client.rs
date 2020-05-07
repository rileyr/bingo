use std::error::Error;

use tokio::prelude::*;
use tokio::runtime::Runtime;
use tokio::net::TcpStream;

use crate::lib::errs;
use crate::lib::reader::Reader;
use crate::lib::messages;

#[derive(Debug)]
pub struct Client {
    addr: String,
    conn: Option<TcpStream>,
}

impl Client {
    pub fn new(addr: String) -> Client {
        Client{addr: addr, conn: None}
    }

    pub async fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        let addr = &self.addr;
        let mut conn = TcpStream::connect(addr).await?;
        self.conn = Some(conn);
        Ok(())
    }

    pub async fn write(&mut self, bytes: &[u8]) -> Result<(), Box<dyn Error>> {
        match &mut self.conn {
            Some(c) => {
                match c.write_all(bytes).await {
                    Err(e) => Err(Box::new(e)),
                    Ok(_) => {
                        println!("sent");
                        Ok(())
                    },
                }
            },
            None => Err(Box::new(errs::StringError::new("not connected"))),
        }
    }

    pub fn example(addr: String) -> Result<(), Box<dyn Error>> {
        let mut client = Client::new(addr);
        let mut rt = Runtime::new()?;

        // Connect!
        rt.block_on(async { client.connect().await; });

        // Start a listener!
        rt.spawn(async move {
        });

        // Send a header!
        rt.block_on(async {
            let hdr = messages::header::new(1, 2, None);
            let mut bts = hdr.to_bytes();
            client.write(bts).await
        });

        Ok(())
    }
}
