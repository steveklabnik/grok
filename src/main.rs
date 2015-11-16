use std::env;
use std::io::BufReader;
use std::fs::File;

extern crate grok;

use grok::Cpu;

fn main() {
    let filename = env::args().skip(1).next().unwrap_or(String::from("sample.asm"));

    let f = File::open(filename).expect("Couldn't find file");
    let reader = BufReader::new(f);

    let mut cpu = Cpu::new();
    cpu.load(reader).expect("Couldn't load instructions");

    cpu.run();

    println!("Final state:\n{}", cpu);
}
