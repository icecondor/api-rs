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
    let peer = setup();
    let mut locations = common::random_locations(1);
    let location = locations.pop().unwrap();
    let location_id = location.id.to_owned();
    let cmd = api::Commands::Write(api::Write {
        id: "ab12".to_owned(),
        params: location,
    });
    let json = serde_json::to_string(&cmd).unwrap();
    let result = peer.command(&json).unwrap();
    assert_eq!("ok", result.msg);

    let cmd = api::Commands::Read(api::Read {
        id: common::id_generate(),
        params: api::QueryById { id: location_id },
    });
    let json = serde_json::to_string(&cmd).unwrap();
    let result = peer.command(&json).unwrap();
    assert_eq!("ok", result.msg);
    let noun = result.noun.unwrap();
    match noun {
        api::Nouns::Location(loc) => {
            assert_eq!("2022-05-02", loc.date)
        }
        _ => assert!(false),
    }
}

#[test]
fn write_many_read_by_id() {
    let peer = setup();
    let locations = common::random_locations(10);
    println!("writing {} locations", locations.len());
    for location in &locations {
        let cmd = api::Commands::Write(api::Write {
            id: common::id_generate(),
            params: location.clone(),
        });
        let json = serde_json::to_string(&cmd).unwrap();
        let result = peer.command(&json).unwrap();
        assert_eq!("ok", result.msg);
    }
    for location in &locations {
        let cmd = api::Commands::Read(api::Read {
            id: common::id_generate(),
            params: api::QueryById {
                id: location.id.to_owned(),
            },
        });
        let json = serde_json::to_string(&cmd).unwrap();
        let result = peer.command(&json).unwrap();
        assert_eq!("ok", result.msg);
    }
}

#[test]
fn auth_by_device() {
    let peer = setup();
    let cmd = api::Commands::AuthBySession(api::AuthByDevice {
        id: common::id_generate(),
        params: api::DeviceId {
            device_id: "devicekey1".to_owned(),
        },
    });
    let json = serde_json::to_string(&cmd).unwrap();
    let result = peer.command(&json).unwrap();
    assert_eq!("ok", result.msg);
}

#[test]
fn auth_by_email() {
    let peer = setup();
    let cmd = api::Commands::AuthByEmail(api::AuthByEmail {
        id: common::id_generate(),
        params: api::Email {
            email: "a@b.c".to_string(),
        },
    });
    let json = serde_json::to_string(&cmd).unwrap();
    let result = peer.command(&json).unwrap();
    assert_eq!("ok", result.msg);
}
