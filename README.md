# Grok

Fun with compilers, interpereters, and such.

## Language

Grok is a few things:

1. A simple language. Integer, plus or minus, then integer.
2. An interpreter/emulator. Kinda looks like a made-up CPU.
3. The assembly/bytecode for that CPU.

## Commands

* `rungrok`: take a grok program and run it
* `grokc`: take a grok program and spit out assembly/bytecode
* `grok`: take assembly/bytecode and execute it
* `grokdb`: take assembly/bytecode and step through like a debugger

## Instructions

`grok` is a machine with three registers, `a`, `b`, and `c`. There’s also a
program counter, `pc`, that you can just observe, not do anything with. Yet.

Instructions:

* `set reg val`: set the register `reg` to the value `val`
* `set reg1 reg2`: set the register `reg1` to the value of `reg2`
* `increment reg`: increment the register `reg`
* `decrement reg`: decrement the register `reg`

That’s it, for now.
