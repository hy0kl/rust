use std::collections::BTreeMap;

pub mod error;
use self::error::Error;

extern crate time;

extern crate ansi_term;
use ansi_term::Colour::{Red, Green, Yellow, Blue, Purple, Cyan};

pub mod api_result;
//use api_result::Body;

extern crate rustc_serialize;
use rustc_serialize::json::{self, ToJson, Json};

fn main() {
    println!("--- {} ---", Red
        //.blink()
        .bold().paint("Test error Model").to_string());

    let code    = Error::get_code(&Error::Success);
    let message = Error::get_message(&Error::Success);
    println!("{{\"code\": {},\"message\":{:?}}}", code, message);

    println!("--- END ---");

    println!("\n--- {} ---", Green.bold().paint("Time").to_string());
    let time_now = time::now();
    println!("{:?}", time_now);

    let time = time::get_time();
    println!("{:?}", time);

    let time_str = time::strftime("%Y-%m-%d %H:%M:%S", &time_now);
    println!("{:?}", time_str);
    println!("--- END ---");

    println!("\n--- {} ---", Yellow.bold().paint("JSON encode decode").to_string());
    let json_str = "{\"code\":0,\"message\":\"OK\",\"data\":{\"keyword\":\"Rust, api, php, nginx\"},\"ip\":[\"192.168.0.168\",\"127.0.0.1\"]}";
    println!("json_str: {}", Blue.paint(json_str.to_string()));
    //let json: api_result::Body = json::decode(&json_str).unwrap();
    let mut obj = BTreeMap::new();
    obj.insert("os".to_string(), "linux".to_string());
    obj.insert("test".to_string(), "array".to_string());
    let json_obj = api_result::Body {
        code: code,
        message: message.to_string(),
        data: obj,
    };
    let encode_str = json_obj.to_json();
    println!("encode_str: {}", encode_str.to_string());
    println!("--- END ---");
}
