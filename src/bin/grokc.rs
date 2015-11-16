use std::env;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    // todo, actual arg handling lol
    let in_filename = env::args().skip(1).next().unwrap_or(String::from("input.grok"));
    let out_filename = env::args().skip(2).next().unwrap_or(String::from("output.asm"));

    let in_f = File::open(in_filename).expect("Couldn't find input file");
    let mut reader = BufReader::new(in_f);

    let mut out_f = File::create(out_filename).expect("Couldn't find output file");

    let mut input = String::new();
    reader.read_line(&mut input).expect("Couldn't read input file");

    let parts: Vec<&str> = input.split_whitespace().collect();

    let a: i32 = parts[0].parse().expect("Couldn't parse the first number.");

    writeln!(out_f, "set a {}", a).expect("Couldn't write to out file");

    let op = match parts[1] {
        "+" => "increment",
        "-" => "decrement",
        _ => panic!("couldn't parse operator"),
    };

    let b = parts[2].parse().expect("Couldn't parse the second number.");

    for _ in 0..b {
        writeln!(out_f, "{} a", op).expect("Couldn't write to out file");
    }
}
