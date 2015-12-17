use std::fmt;
use std::borrow::Cow;
use std::default::Default;

extern crate rustc_serialize;
use rustc_serialize::json;

//#[derive(RustcDecodable, RustcEncodable)]
pub struct UserModel {
    pub id: u64,
    pub nickname: String,
    pub mobile: String,
    //email: RefCell<String>,
    //status: i8,
    //ctime: String,
    //mtime: String,
}

impl UserModel {
    pub fn create_empty() -> UserModel {
        UserModel {
            id: 0,
            nickname: "".to_string(),
            mobile: "".to_string(),
        }
    }

    pub fn create(id: u64, nickname: String, mobile: String) -> UserModel {
        UserModel {
            id: id,
            nickname: nickname,
            mobile: mobile,
        }
    }
}

impl fmt::Display for UserModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf: String = "".to_string();
        buf = buf + &format!("id: {:?}, ", self.id);
        buf = buf + &format!("nickname: \"{}\", ", self.nickname);
        buf = buf + &format!("mobile: \"{}\"", self.mobile);

        write!(f, "UserModel:{{{}}}", buf)
    }
}
