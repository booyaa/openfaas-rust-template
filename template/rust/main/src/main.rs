
use std::io::{self, Read};

extern crate handler;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer)?;
    
    println!("{}", handler::handle(buffer));
    Ok(())
}
