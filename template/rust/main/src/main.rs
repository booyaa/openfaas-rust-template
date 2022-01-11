use std::io;

extern crate handler;

#[tokio::main]
async fn main() -> io::Result<()> {

    let stdin = io::stdin();
    let handle = stdin.lock();

    let req = serde_json::from_reader(handle).expect("Deserializing input");

    let response_obj = handler::handle(req).await;

    let response = serde_json::to_string(&response_obj).unwrap();

    println!("{}", response);

    Ok(())
}
