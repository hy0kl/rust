use std::fs::File;
use std::io::Read;

fn main() {
    let mut data = String::new();
    let mut f = File::open("test.json").ok().expect("Can NOT open file.");
    f.read_to_string(&mut data).ok().expect("Can NOT read file content as string.");

    println!("AS string:");
    println!("{}", data);

    let mut buf = Vec::new();
    let mut f = File::open("test.json").ok().expect("Can NOT open file.");
    f.read_to_end(&mut buf).ok().expect("Can NOT read file content as bytes.");
    println!("{:?}", buf.len());
    println!("AS bytes:");
    println!("{:?}", buf);
}
