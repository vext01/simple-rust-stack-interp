extern crate libsrsi;
use libsrsi::{Interp, fatal};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        fatal("usage: simple-rust-stack-interp <file>");
    } else {
        let mut interp = Interp::new(&args[1]);
        interp.run();
    }
}
