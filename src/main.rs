use std::fs::File;
use std::env;

extern crate javaclass;
extern crate rustop;

use javaclass::read_classfile;
use rustop::opts;

fn main() {
    let (args, rest) = opts! {
        synopsis "A Java virtual machine implementation";
        version "0.0.1";
        opt debug:bool, desc:"be verbose";
        param file:String, desc:"input file";
    }.parse_or_exit();

    let mut file = match File::open(args.file) {
        Ok(s) => s,
        Err(e) => panic!("{:?}", e)
    };

    read_classfile::<File>(&mut file);
}


