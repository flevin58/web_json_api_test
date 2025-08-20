// Practice url: https://jsonplaceholder.typicode.com/users
#![allow(dead_code)]

use reqwest::{self, Method, StatusCode};
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
        .header("User-Agent", "RustApp/1.0 (Macintosh; M4 Mac OS X 15_6)")
        .send();
    let body = match response {
        Ok(resp) => {
            if resp.status() != StatusCode::OK {
                eprintln!("Bad status code {}", resp.status().as_u16());
                return;
            }
            let text = resp.text();
            if text.is_err() {
                eprintln!("Bad response text");
                return;
            }
            text.unwrap()
        }
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
