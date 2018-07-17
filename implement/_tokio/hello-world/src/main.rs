extern crate tokio;

use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;

fn main() {
    let addr = "127.0.0.1:6142".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    let _server = listener.incoming().for_each(|socket| {
        println!("accepted socket; addr={:?}", socket.peer_addr().unwrap());

        Ok(())
    }).map_err(|err| {
        println!("accept error = {:?}", err);
    });

    println!("server running on localhost:6142");
    tokio::run(_server);
}
