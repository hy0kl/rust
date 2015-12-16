pub mod error;
use self::error::Error;

extern crate time;

fn main() {
    println!("--- Test error Model ---");

    let code    = Error::get_code(&Error::Success);
    let message = Error::get_message(&Error::Success);
    println!("{{\"code\": {},\"message\":{:?}}}", code, message);

    println!("--- END ---");

    println!("\n--- Time ---");
    let time_now = time::now();
    println!("{:?}", time_now);

    let time = time::get_time();
    println!("{:?}", time);

    let time_str = time::strftime("%Y-%m-%d %H:%M:%S", &time_now);
    println!("{:?}", time_str);
    println!("--- END ---");
}
