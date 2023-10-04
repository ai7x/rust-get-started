extern crate ferris_says;
use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    // let out = "Hello Rust developer!";
    let out = String::from("Hello Rust developer!");
    let width = 12;
    // let writer = BufWriter::new(stdout());
    let mut writer = BufWriter::new(stdout().lock());
    // say(out, width, writer).unwrap();
    say(&out, width, &mut writer).unwrap();
}
