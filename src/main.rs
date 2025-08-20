// Practice url: https://jsonplaceholder.typicode.com/users
#![allow(dead_code)]

use reqwest::{self, Method};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Geo {
    lat: String,
    lng: String,
}

#[derive(Deserialize, Debug)]
struct Address {
    street: String,
    suite: String,
    city: String,
    zipcode: String,
    geo: Geo,
}

#[derive(Deserialize, Debug)]
struct Company {
    name: String,
    #[serde(rename = "catchPhrase")]
    catchphrase: String,
    bs: String,
}
#[derive(Deserialize, Debug)]
struct User {
    id: usize,
    name: String,
    username: String,
    email: String,
    address: Address,
    phone: String,
    website: String,
    company: Company,
}

fn main() {
    const URL: &str = "https://jsonplaceholder.typicode.com/users";
    let client = reqwest::blocking::Client::new();
    let response = client
        .request(Method::GET, URL)
        .header(
            "User-Agent",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 15_6) Rust-powered",
        )
        .send();
    let body = match response {
        Ok(resp) => resp.text().unwrap(),
        Err(_) => {
            eprintln!("Error getting json data from server");
            return;
        }
    };
    let users: Vec<User> = match serde_json::from_str(&body) {
        Ok(u) => u,
        Err(_) => {
            eprintln!("Error parsing json!");
            return;
        }
    };

    // Display result
    println!("Found {} users:", users.len());
    for (i, user) in users.iter().enumerate() {
        println!("{:>2}: {} <{}>", i + 1, user.name, user.email);
    }
}
