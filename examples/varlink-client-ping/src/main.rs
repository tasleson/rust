#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate varlink;

use org_example_ping::*;
use std::env;
use std::io;
use std::process::exit;

mod org_example_ping;

fn run_app(address: String) -> io::Result<()> {
    let connection = varlink::Connection::new(&address)?;
    let call = VarlinkClient::new(connection);
    let ping: String = "Test".into();

    let reply = call.more().ping(ping.clone())?.recv()?;
    assert_eq!(ping, reply.pong);
    println!("Pong: '{}'", reply.pong);

    let reply = call.more().ping(ping.clone())?.recv()?;
    assert_eq!(ping, reply.pong);
    println!("Pong: '{}'", reply.pong);

    let reply = call.more().ping(ping.clone())?.recv()?;
    assert_eq!(ping, reply.pong);
    println!("Pong: '{}'", reply.pong);

    Ok(())
}

fn main() {
    let mut args: Vec<_> = env::args().collect();
    match args.len() {
        2 => {}
        _ => {
            eprintln!("Usage: {} <varlink address>", args[0]);
            exit(1);
        }
    };

    exit(match run_app(args.swap_remove(1)) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {}", err);
            1
        }
    });
}
