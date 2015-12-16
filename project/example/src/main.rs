pub mod error;

use self::error::Error;

fn main() {
    println!("---Test error Model---");

    let code    = Error::get_code(&Error::Success);
    let message = Error::get_message(&Error::Success);
    println!("{{\"code\": {},\"message\":{:?}}}", code, message);

    println!("--- END ---");
}
