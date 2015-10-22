use std::io::Write;

fn main() {
    //let mut f = std::fs::File::open("foo.txt").write(true).ok().expect("Couldn’t open foo.txt");
    let mut f = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("foo.txt")
        .ok()
        .expect("Couldn’t open foo.txt");

    println!("{:?}", f);

    let buf = b"whatever just study rust.\n";
    let result = f.write(buf).ok().expect("Write fail.");

    println!("ret: {:?}", result);
}
