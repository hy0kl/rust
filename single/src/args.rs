use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();

    let argc = args.len();
    println!("argc: {}", argc);

    for i in 0 .. argc {
        println!("argv[{}]: {}", i, args[i]);
    }
}
