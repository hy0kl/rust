pub enum Error {
    Success,
    Unkown,
}

impl Error {
    pub fn get_code(&self) -> i32 {
        match *self {
            Error::Success => 0,
            Error::Unkown  => -2,
        }
    }

    pub fn get_message(&self) -> &'static str {
        match *self {
            Error::Success => "OK",
            Error::Unkown  => "Undefined error.",
        }
    }
}
