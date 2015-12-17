use std::collections::BTreeMap;

pub mod error;
use self::error::Error;

extern crate time;

extern crate ansi_term;
use ansi_term::Colour::{Red, Green, Yellow, Blue, Purple, Cyan};

pub mod model;
pub mod api_result;

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
    let mut obj: BTreeMap<String, json::Json> = BTreeMap::new();
    obj.insert("os".to_string(), json::Json::from_str("{\"page\": 11, \"os\": \"Linux, Unix, MacOS\"}").unwrap_or_else(|e| { panic!("failed to execute process: {}", e) }));
    obj.insert("test".to_string(), json::Json::from_str("[\"just test\"]").unwrap_or_else(|e| { panic!("failed to execute process: {}", e) }));
    let json_obj = api_result::Body {
        code: code,
        message: message.to_string(),
        data: obj,
    };
    let encode_str = json_obj.to_json();
    println!("encode_str: {}", encode_str.to_string());
    println!("--- END ---");

    println!("\n--- {} ---", Purple.bold().paint("BTreeMap".to_string()));
    // BTreeMap 的 key 只能是同一种类型
    let mut map   = BTreeMap::new();
    let mut n_map = BTreeMap::new();
    n_map.insert(0, "Hi, world");
    n_map.insert(1, "a");
    n_map.insert(2, "b");
    map.insert("key", "value");
    map.insert("os", "linux terminal");
    let n_keys: Vec<_>      = n_map.keys().cloned().collect();
    let n_values: Vec<&str> = n_map.values().cloned().collect();
    let keys: Vec<&str>   = map.keys().cloned().collect();
    let values: Vec<&str> = map.values().cloned().collect();
    println!("n_map->keys:   {:?}", n_keys);
    println!("n_map->values: {:?}", n_values);
    println!("map->keys:   {:?}", keys);
    println!("map->values: {:?}", values);
    for (k, v) in map.iter() {
        println!("map.{} = {}", k, v);
    }
    println!("--- END ---");

    println!("\n--- {} ---", Cyan.bold().paint("UserModel"));
    let empty_user = model::user::UserModel::create_empty();
    println!("empty_user -> {}", empty_user);
    let mut user = model::user::UserModel::create(1, "Admin".to_string(), "15801398759".to_string());
    println!("debug for user: {}", user);
    user.id = 100;
    user.nickname = "OP".to_string();
    println!("debug for user after modify: {}", user);
}
