use std::fmt;
use std::cell::{Cell, RefCell};

extern crate rustc_serialize;
use rustc_serialize::json;

//#[derive(RustcDecodable, RustcEncodable)]
pub struct UserModel {
    id: Cell<u64>,
    //nickname: Cell<Vec<String>>,
    //mobile: RefCell<String>,
    //email: RefCell<String>,
    //status: i8,
    //ctime: String,
    //mtime: String,
}

impl UserModel {
    pub fn new () -> UserModel {
        UserModel {
            id:       Cell::new(0),
            //nickname: Cell::new(vec!("abc".to_string())),
            //mobile:   RefCell::new("".to_string()),
            //email:    RefCell::new("".to_string()),
        }
    }

    pub fn set_id(&self, id: u64) {
        self.id.set(id);
    }
}

impl fmt::Display for UserModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf: String = "".to_string();
        buf = buf + &format!("id: {:?}, ", self.id);

        write!(f, "Hi: {}", buf)
    }
}
