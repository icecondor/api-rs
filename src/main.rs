use std::io::{BufRead, BufReader, Write};
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
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let con = client.get_connection().unwrap();
    let mut peer = peer::new(db, con);
    println!(
        "connected from {} to {}",
        stream.peer_addr().unwrap(),
        stream.local_addr().unwrap()
    );

    // hello
    let hello = api::JsonRPCRequest {
        id: "Bz".to_owned(),
        method: api::Commands::Hello(api::ServerName {
            name: "z".to_owned(),
            version: "1".to_owned(),
        }),
    };
    let mut hello_json = serde_json::to_string(&hello).unwrap();
    hello_json.push_str("\n");
    println!("{}", hello_json);
    stream.write(hello_json.as_bytes()).unwrap();

    let reader = BufReader::new(stream.try_clone().unwrap());
    for line_io in reader.lines() {
        let line = line_io.unwrap();
        println!("{}", line);
        match serde_json::from_str::<api::JsonRPCRequest>(&line) {
            Ok(request) => {
                let result = peer.command(request.method);
                let response = api::JsonRPCResponse {
                    id: request.id,
                    result: result,
                };
                let mut json = serde_json::to_string(&response).unwrap();
                json.push_str("\n");
                println!("{}", json);
                stream.write(json.as_bytes()).unwrap();
            }
            Err(e) => println!("jsonerr: {}", e),
        }
    }
}
