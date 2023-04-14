use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct Password {
    title: String,
    url: String,
    username: String,
    password: String,
    otpauth: String,
    favorite: String,
    tags: String,
    notes: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Identity {
    first_name: String,
    middle_initial: String,
    last_name: String,
    address: String,
    city: String,
    country: String,
    state: String,
    zipcode: i32,
    phone: String,
    email: String,
    apt_number: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Card {
    name: String,
    card_number: String,
    expiration_date: String,
    security_code: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    passwords: Vec<Password>,
    identities: Vec<Identity>,
    cards: Vec<Card>,
}

fn main() {
    // Create sample data
    let data = Data {
        passwords: vec![
            Password {
                title: "Example title".to_string(),
                url: "https://example.com".to_string(),
                username: "username".to_string(),
                password: "password".to_string(),
                otpauth: "otpauth".to_string(),
                favorite: true.to_string(),
                tags: "tags".to_string(),
                notes: "notes".to_string(),
            },
        ],
        identities: vec![
            Identity {
                first_name: "John".to_string(),
                middle_initial: "A".to_string(),
                last_name: "Doe".to_string(),
                address: "123 Main St".to_string(),
                city: "Anytown".to_string(),
                country: "USA".to_string(),
                state: "CA".to_string(),
                zipcode: 12345,
                phone: "1234567890".to_string(),
                email: "john@example.com".to_string(),
                apt_number: "3A".to_string(),
            },
        ],
        cards: vec![
            Card {
                name: "John Doe".to_string(),
                card_number: "1234567812345678".to_string(),
                expiration_date: "12-2025".to_string(),
                security_code: 123,
            },
        ],
    };

    // Serialize data to JSON
    let json_data = serde_json::to_string_pretty(&data).unwrap();

    // Save JSON data to a file
    let mut file = File::create("data.json").unwrap();
    file.write_all(json_data.as_bytes()).unwrap();
}
