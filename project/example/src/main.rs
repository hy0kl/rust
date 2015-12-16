pub mod error;
use self::error::Error;

extern crate time;

extern crate ansi_term;
use ansi_term::Colour::{Red, Green, Yellow, Blue, Purple, Cyan};

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
}
