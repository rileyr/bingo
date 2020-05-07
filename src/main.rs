use std::error::Error;
use std::env;

mod lib;
use lib::server::Server;
use lib::client::Client;
use lib::errs;

const PORT: usize = 3001;
const ADDR: &str = "127.0.0.1:3001";

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            match args[1].as_ref() {
                "server" => return Server::new(PORT).start(),
                "client" => return Client::example(ADDR.to_string()),
                _ => return Err(Box::new(errs::StringError::new("unknown command"))),
            }
        },
        _ => return Err(Box::new(errs::StringError::new("expected one arg"))),
    }
}
