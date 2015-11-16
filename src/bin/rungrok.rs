use std::io;
use std::io::prelude::*;
use std::env;
use std::process::Command;

extern crate tempdir;

use tempdir::TempDir;

fn main() {
    // todo, actual arg handling lol
    let in_filename = env::args().skip(1).next().unwrap_or(String::from("input.grok"));

    let dir = TempDir::new("grok").expect("couldn't create temporary directory");
    let outname = dir.path().join("output.asm");

    let output = Command::new("grokc")
        .arg(in_filename)
        .arg(&outname)
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

    let hello = output.stdout;
    if hello.len() > 0 {
        println!("{}", String::from_utf8_lossy(&hello));
    }

    let output = Command::new("grok")
        .arg(outname)
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

    let hello = output.stdout;
    if hello.len() > 0 {
        print!("{}", String::from_utf8_lossy(&hello));
        io::stdout().flush().expect("Couldn't flush standard out");
    }
}
