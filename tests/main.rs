use api_rs::*;

use std::sync::Arc;

mod common;

fn setup() -> peer::Peer {
    let db = db::open();
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let con = client.get_connection().unwrap();
    peer::new(Arc::new(db), con)
}

#[test]
fn write_one_read_one() {
    let mut peer = setup();
    let mut locations = common::random_locations(1);
    let location = locations.pop().unwrap();
    let location_id = location.id.to_owned();
    let cmd = api::Commands::Write(api::Write {
        id: common::id_generate(),
        params: location,
    });
    let result = peer.command(cmd);
    match result {
        api::Response::Result(_) => assert!(true),
        api::Response::Error(_) => assert!(false),
    }

    let cmd = api::Commands::Read(api::Read {
        id: common::id_generate(),
        params: api::ById { id: location_id },
    });
    let result = peer.command(cmd);
    match result {
        api::Response::Result(noun) => match noun {
            api::Nouns::Location(loc) => {
                assert_eq!("2022-05-02", loc.date)
            }
            _ => assert!(false),
        },
        api::Response::Error(_) => assert!(false),
    }
}

#[test]
fn write_many_read_by_id() {
    let mut peer = setup();
    let locations = common::random_locations(10);
    println!("writing {} locations", locations.len());
    for location in &locations {
        let cmd = api::Commands::Write(api::Write {
            id: common::id_generate(),
            params: location.clone(),
        });
        let result = peer.command(cmd);
        match result {
            api::Response::Result(_) => assert!(true),
            api::Response::Error(_) => assert!(false),
        }
    }
    for location in &locations {
        let cmd = api::Commands::Read(api::Read {
            id: common::id_generate(),
            params: api::ById {
                id: location.id.to_owned(),
            },
        });
        let result = peer.command(cmd);
        match result {
            api::Response::Result(_) => assert!(true),
            api::Response::Error(_) => assert!(false),
        }
    }
}

#[test]
fn auth_by_device() {
    let mut peer = setup();
    let cmd = api::Commands::AuthBySession(api::DeviceKey {
        device_key: "devicekey1".to_owned(),
    });
    let result = peer.command(cmd);
    match result {
        api::Response::Result(_) => assert!(true),
        api::Response::Error(_) => assert!(false),
    }
}

#[test]
fn auth_by_email() {
    let mut peer = setup();
    let cmd = api::Commands::AuthByEmail(api::Email {
        email: "a@b.c".to_string(),
    });
    let result = peer.command(cmd);
    match result {
        api::Response::Result(_) => assert!(true),
        api::Response::Error(_) => assert!(false),
    }
}
