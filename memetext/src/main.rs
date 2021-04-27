// use std::io;
use std::env;

fn main() {
    println!("Beginning program!");
    let args: Vec<String> = env::args().collect();
    let mut res: Vec<String> = Vec::new();
    for n in 1..args.len() {
        let my_string: String = &args[n];
        res.push(my_string);
    }
    println!("{:?}", res);
}
