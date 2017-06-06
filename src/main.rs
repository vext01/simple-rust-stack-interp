#![allow(warnings)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::process::exit;

type Program = Vec<Instr>;
type LabelMap = HashMap<String, usize>;
type RawNumber = i32;

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

    fn fatal(msg: &str) {
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
        (vec![], LabelMap::new())
    }

    fn parse_line(line: String) -> Result<Instr, ()> {
        let line = line.trim();
        let rv = match line {
            "add" => Instr::Add,
            "print" => Instr::Print,
            _ => {
                let num = line.parse::<RawNumber>();
                if num.is_err() {
                    return Err(());
                }
                let num = num.unwrap();
                Instr::Push(StackVal::Number(num))
            }
        };
        Ok(rv)
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
            _ => {
                Self::fatal("type mismatch");
                unreachable!();
            }
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
    JumpEqual,
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
