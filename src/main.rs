use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::fmt;

#[derive(Debug)]
struct Cpu {
    instructions: Vec<Instruction>,
    pc: usize, // lol
    a: u64,
    b: u64,
    c: u64,
}

#[derive(Debug)]
#[derive(Copy,Clone)]
enum Register {
    A,
    B,
    C,
}

#[derive(Debug)]
#[derive(Copy,Clone)]
enum Instruction {
    Set(Register, Value),
    Increment(Register),
    Decrement(Register),
}

#[derive(Debug)]
#[derive(Copy,Clone)]
enum Value {
    Immediate(u64),
    Register(Register),
}

fn main() {
    let mut cpu = Cpu::new();

    let f = File::open("sample.asm").expect("Couldn't find file");
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line.unwrap();
        if let Ok(instruction) = parse_line(&line) {
            cpu.instructions.push(instruction);
        }
    }

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

impl Cpu {
    fn new() -> Cpu {
        Cpu { instructions: Vec::new(), pc: 0, a: 0, b: 0, c: 0 }
    }

    fn finished(&self) -> bool {
        self.pc >= self.instructions.len()
    }

    fn run(&mut self) {
        loop {
            if !self.finished() {
                self.step();
            } else {
                break;
            }
        }
    }

    fn step(&mut self) {
        if self.finished() {
            return;
        }

        match self.instructions[self.pc] {
            Instruction::Set(register, value) => {
                let value = match value {
                    Value::Immediate(number) => number,
                    Value::Register(register) => match register {
                        Register::A => self.a,
                        Register::B => self.b,
                        Register::C => self.c,
                    }
                };

                match register {
                    Register::A => self.a = value,
                    Register::B => self.b = value,
                    Register::C => self.c = value,
                }
            },
            Instruction::Increment(register) => {
                match register {
                    Register::A => self.a += 1,
                    Register::B => self.b += 1,
                    Register::C => self.c += 1,
                }
            },
            Instruction::Decrement(register) => {
                match register {
                    Register::A => self.a -= 1,
                    Register::B => self.b -= 1,
                    Register::C => self.c -= 1,
                }
            },
        }
        
        self.pc += 1;
    }
}

impl fmt::Display for Cpu {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, "Registers:"));
        try!(writeln!(f, "a:\t{}", self.a));
        try!(writeln!(f, "b:\t{}", self.b));

        write!(f, "c:\t{}", self.c)
    }
}

fn parse_line(line: &str) -> Result<Instruction, ()> {
    let parts: Vec<_> = line.split_whitespace().collect(); 

    // i suck at parsing, so sorry
    let register = match parts[1] {
        "a" => Register::A,
        "b" => Register::B,
        "c" => Register::C,
        _ => return Err(()), 
    };

    match parts[0] {
        "set" => {
            let value = match parts[2].parse() {
                Ok(value) => Value::Immediate(value),
                Err(_) => {
                    let register = match parts[2] {
                        "a" => Register::A,
                        "b" => Register::B,
                        "c" => Register::C,
                        _ => return Err(()), 
                    };

                    Value::Register(register)
                },
            };

            Ok(Instruction::Set(register, value))
        },
        "increment" => {
            Ok(Instruction::Increment(register))
        },
        "decrement" => {
            Ok(Instruction::Decrement(register))
        },
        _ => Err(())
    }
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
