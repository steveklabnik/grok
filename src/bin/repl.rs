use std::env;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

extern crate grok;

use grok::Cpu;

fn main() {
    let filename = env::args().skip(1).next().unwrap_or(String::from("sample.asm"));

    let f = File::open(filename).expect("Couldn't find file");
    let reader = BufReader::new(f);

    let mut cpu = Cpu::new();
    cpu.load(reader);

    while let Ok(input) = fetch_line() {
        if input == "s" {
            cpu.step();
        } else if input == "c" {
            cpu.run();
        } else if input == "p" {
            println!("{:?}", cpu);
        }

        if cpu.finished() {
            break;
        }
    }

    println!("Final state:\n{}", cpu);
}

fn fetch_line() -> io::Result<String> {
    let mut input = String::new();

    print!("> ");
    try!(io::stdout().flush());

    try!(io::stdin().read_line(&mut input));

    // trim newline
    let new_len = input.len() - 1;
    input.truncate(new_len);

    Ok(input)
}
