pub enum Error {
    Unkown,
    Success,
}

impl Error {
    pub fn get_code(&self) -> i32 {
        match *self {
            Error::Unkown  => -1,
            Error::Success => 0,
        }
    }

    pub fn get_message(&self) -> &'static str {
        match *self {
            Error::Unkown  => "Undefined error.",
            Error::Success => "OK",
        }
    }
}
