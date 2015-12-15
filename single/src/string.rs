fn main() {
    let mut s = "Hello".to_string(); // mut s: String
    println!("{}", s);

    s.push_str(", world.");
    println!("{}", s);

    let hachiko = "忠犬ハチ公";

    for b in hachiko.as_bytes() {
        print!("{}, ", b);
    }

    println!("");

    for c in hachiko.chars() {
        print!("{}, ", c);
    }

    println!("");

    let dog = hachiko.chars().nth(1); // kinda like hachiko[1]
    println!("dog: {:?}", dog);

    let hello = "Hello ".to_string();
    let world = "world!";

    let hello_world = hello + world;
    println!("strcat no &: {}", hello_world);

    let hello = "Hello ".to_string();
    let world = "world!".to_string();

    let hello_world = hello + &world;
    println!("strcat take &: {}", hello_world);
}
