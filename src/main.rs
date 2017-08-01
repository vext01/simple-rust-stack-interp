#![feature(metarust)]
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::process::exit;
use std::{env, jit_merge_point};

type Program = Vec<Instr>;
type LabelMap = HashMap<String, usize>;
type RawNumber = i32;
type LabelName = String;

struct Interp {
    program: Program,
    labels: LabelMap,
    stack: Stack,
    pc: usize,
}

impl Interp {
    fn new(filename: &str) -> Self {
        let (program, labels) = Self::parse(filename);
        Self {
            program: program,
            labels: labels,
            stack: Stack::new(),
            pc: 0,
        }
    }

    fn fatal(msg: &str) -> ! {
        println!("FATAL: {}", msg);
        exit(1);
    }

    fn parse(filename: &str) -> (Program, LabelMap) {
        // Get ready to iterate over the source program
        let fh = File::open(filename);
        if fh.is_err() {
            Self::fatal(&format!("Failed to open input file: {}", filename));
        }
        let fh = fh.unwrap();
        let reader = BufReader::new(fh);

        let mut program = Program::new();
        let mut labels = LabelMap::new();
        for line in reader.lines() {
            match Self::parse_line(line.unwrap()) {
                ParsedLine::Instr(instr) => program.push(instr),
                ParsedLine::Label(label) => {
                    if labels.insert(label, program.len()).is_some() {
                        Self::fatal("parse error: duplicate label");
                    }
                }
            }
        }
        (program, labels)
    }

    fn parse_number<'a>(s: &'a str) -> RawNumber {
        let num = s.parse::<RawNumber>();
        if num.is_err() {
            Self::fatal("parse error: unparsed number");
        }
        num.unwrap()
    }

    fn parse_line(line: String) -> ParsedLine {
        let line = line.trim();
        let mut operands = line.split(" ").map(|x| x.trim());

        let rv = {
            let mut next_operand = || match operands.next() {
                Some(s) => s.trim(),
                None => {
                    Self::fatal("parse error: too few arguments");
                }
            };

            let opcode = next_operand();
            let rv = match opcode {
                "add" => ParsedLine::Instr(Instr::Add),
                "sub" => ParsedLine::Instr(Instr::Sub),
                "print" => ParsedLine::Instr(Instr::Print),
                "pop" => ParsedLine::Instr(Instr::Pop),
                "dup" => ParsedLine::Instr(Instr::Dup),
                "je" => {
                    let cmp_val = Self::parse_number(next_operand());
                    let target = next_operand();
                    ParsedLine::Instr(Instr::JumpEqual(cmp_val, String::from(target)))
                }
                "jne" => {
                    let cmp_val = Self::parse_number(next_operand());
                    let target = next_operand();
                    ParsedLine::Instr(Instr::JumpNotEqual(cmp_val, String::from(target)))
                }
                "push" => {
                    let val = Self::parse_number(next_operand());
                    ParsedLine::Instr(Instr::Push(StackVal::Number(val)))
                }
                _ => {
                    if opcode.ends_with(":") {
                        // XXX in a real interpreter you would resolve the labels to addresses
                        // ahead of time so that: a) a bad label is compile-time detected, and b)
                        // you don't have to repeatedly look them up.
                        ParsedLine::Label(opcode[..opcode.len() - 1].to_owned())
                    } else {
                        Self::fatal("parse error: unknown opcode");
                    }
                }
            };
            rv
        };
        if operands.next().is_some() {
            Self::fatal("parse error: too many operands");
        }
        rv
    }

    fn run(&mut self) {
        // main interpreter loop
        loop {
            jit_merge_point(self.pc);
            let instr = self.program.get(self.pc);
            if instr.is_none() {
                return; // end of program
            }

            match instr.unwrap() {
                &Instr::Push(ref val) => {
                    self.stack.push(val.clone());
                    self.pc += 1;
                }
                &Instr::Add => {
                    let (arg1, arg2) = (self.stack.pop_number(), self.stack.pop_number());
                    self.stack.push(StackVal::Number(arg1 + arg2));
                    self.pc += 1;
                }
                &Instr::Dup => {
                    let val = self.stack.pop();
                    self.stack.push(val.clone());
                    self.stack.push(val);
                    self.pc += 1;
                }
                &Instr::Sub => {
                    let (arg1, arg2) = (self.stack.pop_number(), self.stack.pop_number());
                    self.stack.push(StackVal::Number(arg2 - arg1));
                    self.pc += 1;
                }
                &Instr::Print => {
                    let arg = self.stack.pop_number();
                    println!("{}", arg);
                    self.pc += 1;
                }
                &Instr::Pop => {
                    let _ = self.stack.pop();
                    self.pc += 1;
                }
                // XXX generalise binary operations to reduce duplication
                &Instr::JumpNotEqual(ref cmp_val, ref label) => {
                    let val = self.stack.pop_number();
                    if val != *cmp_val {
                        if let Some(addr) = self.labels.get(label) {
                            self.pc = *addr;
                        } else {
                            Self::fatal("undefined label");
                        }
                    } else {
                        self.pc += 1;
                    }
                }
                &Instr::JumpEqual(ref cmp_val, ref label) => {
                    let val = self.stack.pop_number();
                    if val == *cmp_val {
                        if let Some(addr) = self.labels.get(label) {
                            self.pc = *addr;
                        } else {
                            Self::fatal("undefined label");
                        }
                    } else {
                        self.pc += 1;
                    }
                }
            }
        }
    }
}

#[derive(Clone)]
enum Instr {
    Push(StackVal),
    Pop,
    Add,
    Dup,
    Sub,
    JumpEqual(RawNumber, LabelName), // jump to .1 if top of stack == .0
    JumpNotEqual(RawNumber, LabelName), // jump to .1 if top of stack != .0
    Print,
}

#[derive(Clone)]
enum ParsedLine {
    Label(LabelName),
    Instr(Instr),
}

#[derive(Clone)]
enum StackVal {
    Number(RawNumber),
}

struct Stack {
    stack: Vec<StackVal>,
}

impl Stack {
    fn new() -> Self {
        Stack { stack: vec![] }
    }

    fn push(&mut self, val: StackVal) {
        self.stack.push(val);
    }

    fn pop(&mut self) -> StackVal {
        let val = self.stack.pop();
        if val.is_none() {
            Interp::fatal("stack underflow");
        }
        val.unwrap()
    }

    fn pop_number(&mut self) -> RawNumber {
        let item = self.pop();
        let rv = match item {
            StackVal::Number(val) => val,
        };
        rv
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        Interp::fatal("usage: simple-rust-stack-interp <file>");
    } else {
        let mut interp = Interp::new(&args[1]);
        interp.run();
    }
}
