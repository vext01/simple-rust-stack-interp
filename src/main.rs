#![allow(warnings)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::process::exit;

type Program = Vec<Instr>;
type LabelMap = HashMap<String, usize>;
type RawNumber = i32;
type LabelName = String;

struct Interp {
    program: Program,
    labels: LabelMap,
    stack: Vec<StackVal>,
    pc: usize,
}

impl Interp {
    fn new(filename: &str) -> Self {
        let (program, labels) = Self::parse(filename);
        Self {
            program: program,
            labels: labels,
            stack: vec![],
            pc: 0,
        }
    }

    fn fatal(msg: &str) -> ! {
        println!("FATAL: {}", msg);
        exit(1);
    }

    fn parse(filename: &str) -> (Program, LabelMap) {
        let fh = File::open(filename);
        if fh.is_err() {
            Self::fatal(&format!("Failed to open input file: {}", filename));
        }
        let fh = fh.unwrap();

        let reader = BufReader::new(fh);
        let mut program = Program::new();
        for line in reader.lines() {
            match Self::parse_line(line.unwrap()) {
                Ok(instr) => program.push(instr),
                Err(_) => Self::fatal("parse error"),
            }
        }
        (program, LabelMap::new())
    }

    fn parse_number<'a>(s: &'a str) -> RawNumber {
        let num = s.parse::<RawNumber>();
        if num.is_err() {
            Self::fatal("parse error: unparsed number");
        }
        num.unwrap()
    }

    fn parse_line(line: String) -> Result<Instr, ()> {
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
                "add" => Instr::Add,
                "sub" => Instr::Sub,
                "print" => Instr::Print,
                "pop" => Instr::Pop,
                "je" => {
                    let cmp_val = Self::parse_number(next_operand());
                    let target = next_operand();
                    Instr::JumpEqual(cmp_val, String::from(target))
                }
                "push" => {
                    let val = Self::parse_number(next_operand());
                    Instr::Push(StackVal::Number(val))
                }
                _ => {
                    Self::fatal("parse error: unknown opcode");
                }
            };
            Ok(rv)
        };
        if operands.next().is_some() {
            Self::fatal("parse error: too many operands");
        }
        rv
    }

    fn push(&mut self, val: StackVal) {
        self.stack.push(val);
    }

    fn pop(&mut self) -> StackVal {
        let val = self.stack.pop();
        if val.is_none() {
            Self::fatal("stack underflow");
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

    fn run(&mut self) {
        // main interpreter loop
        let program: Vec<Instr> = self.program.to_owned();
        loop {
            let instr = program.get(self.pc);
            if instr.is_none() {
                return; // end of program
            }

            match instr.unwrap() {
                &Instr::Push(ref val) => {
                    self.push(val.clone());
                    self.pc += 1;
                }
                &Instr::Add => {
                    let (arg1, arg2) = (self.pop_number(), self.pop_number());
                    self.push(StackVal::Number(arg1 + arg2));
                    self.pc += 1;
                }
                &Instr::Sub => {
                    let (arg1, arg2) = (self.pop_number(), self.pop_number());
                    self.push(StackVal::Number(arg1 - arg2));
                    self.pc += 1;
                }
                &Instr::Print => {
                    let arg = self.pop_number();
                    println!("{}", arg);
                    self.pc += 1;
                }
                &Instr::Pop => {
                    let _ = self.pop();
                    self.pc += 1;
                }
                _ => Self::fatal("Not implemented"),
            }
        }
    }
}

#[derive(Clone)]
enum Instr {
    Push(StackVal),
    Pop,
    Add,
    Sub,
    JumpEqual(RawNumber, LabelName), // jump to .1 if top of stack == .0
    Print,
}

#[derive(Clone)]
enum StackVal {
    Number(RawNumber),
}

fn main() {
    let mut interp = Interp::new("test.iin");
    interp.run();
}
