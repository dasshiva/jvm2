use std::fs::File;

extern crate javaclass;
extern crate rustop;
extern crate log;
mod logger;

use javaclass::read_classfile;
use rustop::opts;
use log::LevelFilter;
use std::process::exit;
use log::*;

fn main() {
    let (args, rest) = opts! {
        synopsis "A Java virtual machine implementation.";
        version "0.0.1";
        opt verbose:bool, desc:"Be verbose.";
        param file:String, desc:"Input file.";
    }.parse_or_exit();

    if args.verbose {
        logger::init(LevelFilter::Trace);
    }
    else {
        logger::init(LevelFilter::Error);
    }

    debug!("Starting VM");
    debug!("Opening file {}", args.file);
    let mut file = match File::open(args.file) {
        Ok(s) => s,
        Err(e) => exit!("{}", e) ,
    };

   debug!("Reading class file");
   let class = match read_classfile::<File>(&mut file) {
        Ok(s) => s,
        Err(e) => exit!("{}", e),
   }; 

   if class.major_version > 52 {
       exit!("This JVM only supports class files upto version 52 (Java 1.8) but the given file has version {} (Java 1.{})", class.major_version, class.major_version - 44);
   }

}


