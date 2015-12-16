extern crate rustc_serialize;

use std::collections::BTreeMap;
use rustc_serialize::json::{self, ToJson, Json};

//#[derive(RustcDecodable, RustcEncodable)]
//#[derive(RustcDecodable)]
pub struct Body {
    pub code: i64,
    pub message: String,
    pub data: BTreeMap<String, String>, // BTreeMap 类型编译器无法自动实现
}

impl ToJson for Body {
    // 实现特性时不需要再使用 pub 关键字
    fn to_json(&self) -> Json {
        let mut json_obj = BTreeMap::new();
        // All standard types implement `to_json()`, so use it
        json_obj.insert("code".to_string(), self.code.to_json());
        json_obj.insert("message".to_string(), self.message.to_json());
        json_obj.insert("data".to_string(), self.data.to_json());

        Json::Object(json_obj)
    }
}
