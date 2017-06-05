#![allow(warnings)]

use std::collections::HashMap;

type Program = Vec<Instr>;
type LabelMap = HashMap<String, u32>;

struct Interp {
    program: Program,
    labels: LabelMap,
    stack: Vec<StackVal>,
}

impl Interp {
    fn new(filename: &str) -> Self {
        let (program, labels) = Self::parse(filename);
        Self {
            program: program,
            labels: labels,
            stack: vec![],
        }
    }

    fn parse(filename: &str) -> (Program, LabelMap) {
        (vec![], LabelMap::new())
    }

    fn run(&mut self) {}
}

enum Instr {
    Push(StackVal),
    Pop,
    Add,
    Sub,
    JumpEqual,
    Print,
}

enum StackVal {
    Number(u32),
}

fn main() {
    let mut interp = Interp::new("test.iin");
    interp.run();
}
