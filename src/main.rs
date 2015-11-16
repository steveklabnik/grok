use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

#[derive(Debug)]
struct Registers {
    pc: u64,
    a: u64,
    b: u64,
    c: u64,
}

#[derive(Debug)]
enum Register {
    A,
    B,
    C,
}

enum Operation {
    Set(Register, Value),
    Increment(Register),
    Decrement(Register),
}

enum Value {
    Immediate(u64),
    Register(Register),
}

fn main() {
    let mut registers = Registers { pc: 0, a: 0, b: 0, c: 0 };
    let mut codes: Vec<Operation> = Vec::new();

    let f = File::open("sample.asm").expect("Couldn't find file");
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line.unwrap();
        if let Ok(operation) = parse_line(&line) {
            codes.push(operation);
        }
    }

    for operation in codes {
        registers.apply(operation);
            println!("{:?}", registers);
    }
}

impl Registers {
    fn apply(&mut self, operation: Operation) {
        match operation {
            Operation::Set(register, value) => {
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
            Operation::Increment(register) => {
                match register {
                    Register::A => self.a += 1,
                    Register::B => self.b += 1,
                    Register::C => self.c += 1,
                }
            },
            Operation::Decrement(register) => {
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

fn parse_line(line: &str) -> Result<Operation, ()> {
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

            Ok(Operation::Set(register, value))
        },
        "increment" => {
            Ok(Operation::Increment(register))
        },
        "decrement" => {
            Ok(Operation::Decrement(register))
        },
        _ => Err(())
    }
}
