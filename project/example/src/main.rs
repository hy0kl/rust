use std::default::Default;

extern crate time;

extern crate ansi_term;
use ansi_term::Colour::{Red, Green, Yellow, Blue, Purple, Cyan};

extern crate mysql;
use mysql::conn::MyOpts;
use mysql::conn::pool::MyPool;
use mysql::value::from_row;
use std::collections::BTreeMap;

pub mod error;
use self::error::Error;


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
    // error: cannot assign to immutable field
    //empty_user.id = 110;
    println!("empty_user -> {}", empty_user);
    let mut user = model::user::UserModel::create(1, "Admin".to_string(), "15801398759".to_string(), "abc@qq.com".to_string());
    println!("debug for user: {}", user);
    user.id = 100;
    user.nickname = "OP".to_string();
    println!("debug for user after modify: {}", user);
    let user_json = json::encode(&user).unwrap();
    println!("user_json: {}", user_json);

    // CREATE TABLE  `test`.`user` (
    //  `id` INT( 11 ) UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY COMMENT  '主键',
    //  `nickname` VARCHAR( 128 ) NOT NULL ,
    //  `mobile` VARCHAR( 32 ) NOT NULL ,
    //  `email` VARCHAR( 64 ) NOT NULL
    // ) ENGINE = INNODB;
    println!("\n--- {} ---", Red.bold().paint("MySQL example".to_string()));
    let opts = MyOpts {
        user: Some("dev".to_string()),
        pass: Some("dev".to_string()),
        //init: vec!["SET NAMES utf8; use test;".to_owned()], // 加入此行无法写数据.
        ..Default::default()
    };
    let pool = MyPool::new(opts).unwrap();
    //let method = "insert";
    let method = "select";
    if "insert" == method {
        println!("{}", Yellow.blink().bold().paint("INSERT"));
        let mut users = vec![
            model::user::UserModel{id : 1, nickname: "admin".to_string(), mobile: "15811119890".to_string(), email: "158@qq.com".to_string()},
        ];
        users.push(user);
        for mut stmt in pool.prepare(r"INSERT INTO test.user
                (id, nickname, mobile, email)
                VALUES
                (NULL, ?, ?, ?)").into_iter() {
            for p in users.iter() {
                // `execute` takes ownership of `params` so we pass account name by reference.
                // Unwrap each result just to make sure no errors happended.
                let _ig = stmt.execute((&p.nickname, &p.mobile, &p.email));
            }
        }
    } else {
        println!("{}", Yellow.blink().bold().paint("SELECT"));
        let selected: Vec<model::user::UserModel> = pool.prep_exec("SELECT id, nickname, mobile, email FROM test.user", ())
            .map(|result| { // In this closure we sill map `QueryResult` to `Vec<T>`
                // `QueryResult` is iterator over `MyResult<row, err>` so first call to `map`
                // will map each `MyResult` to contained `row` (no proper error handling)
                // and second call to `map` will map each `row` to `struct`
                result.map(|x| x.unwrap()).map(|row| {
                    let (id, nickname, mobile, email) = from_row(row);
                    model::user::UserModel {
                        id: id,
                        nickname: nickname,
                        mobile: mobile,
                        email: email,
                    }
                }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<T>`
        }).unwrap(); // Unwrap `Vec<T>`
        for i in 0 .. selected.len() {
            println!("selected[{}]: {}", i, selected[i]);
        }
    }
}
