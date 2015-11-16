use std::io;

#[derive(Debug)]
struct Registers {
    a: u64,
    b: u64,
    c: u64,
}

enum Register {
    A,
    B,
    C,
}

enum Operation {
    Set(Register)
}

fn main() {
    let mut registers = Registers { a: 0, b: 0, c: 0 };

    while let Ok(input) = fetch_input() {
        if let Ok(operation) = parse_input(&input) {
            match operation {
                Operation::Set(register) => {
                    match register {
                        Register::A => registers.a = 5,
                        Register::B => registers.b = 6,
                        Register::C => registers.c = 7,
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

    if parts[0] == "set" {
        match parts[1] {
            "a" => Ok(Operation::Set(Register::A)),
            "b" => Ok(Operation::Set(Register::B)),
            "c" => Ok(Operation::Set(Register::C)),
            _ => Err(()), 
        }
    } else {
        Err(())
    }

}

fn fetch_input() -> io::Result<String> {
    let mut input = String::new();

    try!(io::stdin().read_line(&mut input));

    // trim newline
    let new_len = input.len() - 1;
    input.truncate(new_len);

    Ok(input)
}
