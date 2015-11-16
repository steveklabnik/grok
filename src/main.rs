use std::io;
use std::io::prelude::*;

#[derive(Debug)]
struct Registers {
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
    let mut registers = Registers { a: 0, b: 0, c: 0 };

    while let Ok(input) = fetch_input() {
        if let Ok(operation) = parse_input(&input) {
            match operation {
                Operation::Set(register, value) => {
                    let value = match value {
                        Value::Immediate(number) => number,
                        Value::Register(register) => match register {
                            Register::A => registers.a,
                            Register::B => registers.b,
                            Register::C => registers.c,
                        }
                    };

                    match register {
                        Register::A => registers.a = value,
                        Register::B => registers.b = value,
                        Register::C => registers.c = value,
                    }
                },
                Operation::Increment(register) => {
                    match register {
                        Register::A => registers.a += 1,
                        Register::B => registers.b += 1,
                        Register::C => registers.c += 1,
                    }
                },
                Operation::Decrement(register) => {
                    match register {
                        Register::A => registers.a -= 1,
                        Register::B => registers.b -= 1,
                        Register::C => registers.c -= 1,
                    }
                },
            }
            println!("{:?}", registers);
        }
    }
}

fn parse_input(input: &str) -> Result<Operation, ()> {
    let parts: Vec<_> = input.split_whitespace().collect(); 

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

fn fetch_input() -> io::Result<String> {
    let mut input = String::new();

    print!("> ");
    try!(io::stdout().flush());

    try!(io::stdin().read_line(&mut input));

    // trim newline
    let new_len = input.len() - 1;
    input.truncate(new_len);

    Ok(input)
}
