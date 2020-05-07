use std::error::Error;
use std::fmt;

use tokio::prelude::*;
use tokio::runtime::Runtime;
use tokio::net::{TcpListener, TcpStream};

use crate::lib::messages::header;
use crate::lib::errs;
use crate::lib::reader::Reader;

pub struct Server {
    port: usize,
    conn_count: usize,
}

struct Handler {
    conn: TcpStream,
    id: usize,
}

impl Server {
    pub fn new(port: usize) -> Server {
        Server{
            port: port,
            conn_count: 0,
        }
    }

    pub fn start(&mut self) -> Result<(), Box<dyn Error>> {
        let mut rt = Runtime::new().unwrap();
        rt.block_on(async{ self.listen().await })
    }

    async fn listen(&mut self) -> Result<(), Box<dyn Error>> {
        let addr = format!("127.0.0.1:{}", self.port);
        println!("server listening at {}", addr);
        let mut listener = TcpListener::bind(addr).await?;
        loop {
            let (conn, _) = listener.accept().await?;
            self.conn_count = self.conn_count + 1;
            let id = self.conn_count;
            println!("new connection");
            tokio::spawn(async move {
                match Handler::new(id, conn).handle().await {
                    Err(e) => return Err(Box::new(errs::StringError::new(&format!("handler error").to_string()))),
                    _ => return Ok(()),
                }
            });
        }
    }
}

impl Handler{
    pub fn new(id: usize, conn: TcpStream) -> Handler {
        Handler{
            conn: conn,
            id: id,
        }
    }

    async fn handle(self) -> Result<(), Box<dyn Error>> {
        println!("starting handler {}", self.id);
        let (rr, _rw) = self.conn.split();
        let mut rd = Reader::new(rr);

        loop {
            let m = rd.next_message().await.expect("read failed");
            println!("got message: {}", m);
        }
    }
}

impl fmt::Display for Server {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "bingo server\nport: {}", self.port)
    }
}
