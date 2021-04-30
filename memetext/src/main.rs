// use std::io;
use std::env;
use std::io::{self, Write};
use rand::Rng;

fn meme_text(text: String) -> String {
    let mut to_return: String = "".to_owned();
    let mut rng = rand::thread_rng();
    for i in 0..text.len() {
        let num: f32 = rng.gen::<f32>();
        if num < 0.5 {
            to_return.push_str(&text.chars().nth(i).unwrap().to_string().to_uppercase());
        } else {
            to_return.push_str(&text.chars().nth(i).unwrap().to_string().to_lowercase());
        }
    }
    return to_return;
}

fn args_to_string(args: &Vec<String>) -> String {
    let mut to_return: String = "".to_owned();
    for i in 1..args.len() {
        to_return.push_str(&args[i]);
        if i < args.len() - 1 {
            to_return.push_str(" ");
        }
    }
    return to_return;
}

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let args: Vec<String> = env::args().collect();
    // println!("{}", meme_text(args_to_string(&args)));
    handle.write_all(meme_text(args_to_string(&args)).as_bytes())?;
    Ok(())
}
