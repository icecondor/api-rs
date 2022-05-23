use std::io::{BufReader, BufRead, Write};
use std::net::TcpStream;
use std::sync::Arc;

use api_rs::*;

fn main() {
    let config = config::load();
    let db = Arc::new(db::open());

    let net = net::setup(&config.addr);
    let mut pool = pool::new();

    println!("listening {}", config.addr);
    for stream in net.listener.incoming() {
        let dbc = db.clone();
        match stream {
            Err(_) => println!("socket accept err"),
            Ok(stream) => pool.push(move || peer_reader(stream, dbc)),
        }
        println!("threadpool size {}", pool.len())
    }
}

fn peer_reader(mut stream: TcpStream, db: Arc<db::Db>) {
    let peer = peer::new(db);
    println!(
        "connected from {} to {}",
        stream.peer_addr().unwrap(),
        stream.local_addr().unwrap()
    );
    let reader = BufReader::new(stream.try_clone().unwrap());
    for line in reader.lines() {
        let result = peer.command(&line.unwrap()).unwrap();
        let mut json = serde_json::to_string(&result).unwrap();
        json.push_str("\n");
        stream.write(json.as_bytes()).unwrap();
    }
}
