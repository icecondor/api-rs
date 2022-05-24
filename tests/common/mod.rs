use crate::nouns;
use uuid::Uuid;

pub fn id_generate() -> String {
    Uuid::new_v4().to_string()
}

pub fn random_locations(count: u32) -> Vec<nouns::location::Location> {
    let mut bag = vec![];
    for _ in 0..count {
        let mut location = nouns::location::Location::new();
        location.id = id_generate();
        location.latitude = 1.0;
        location.longitude = 2.0;
        location.accuracy = 2.0;
        location.altitude = 2.0;
        location.date = "2022-05-02".to_owned();
        location.device_id = "bas21".to_owned();
        location.user_id = "Abc".to_owned();
        location.provider = "gps".to_owned();
        location.received_at = "2022-05-23T12:12:00".to_owned();
        bag.push(location);
    }
    return bag;
}
