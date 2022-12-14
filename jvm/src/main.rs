use std::fs::File;

extern crate javaclass;
extern crate log;
extern crate rustop;
mod frame;
mod loader;
mod logger;
mod primarray;

use frame::Frame;
use javaclass::read_classfile;
use log::LevelFilter;
use log::*;
use rustop::opts;

fn main() {
    let (args, _) = opts! {
        synopsis "A Java virtual machine implementation.";
        version "0.0.1";
        opt verbose:bool, desc:"Be verbose.";
        //opt path:String, desc:"Specify the classpath.";
        param file:String, desc:"Input file.";
    }
    .parse_or_exit();

    if args.verbose {
        logger::init(LevelFilter::Trace);
    } else {
        logger::init(LevelFilter::Error);
    }

    debug!("Starting VM");
    debug!("Opening file {}", args.file);
    let mut file = match File::open(args.file) {
        Ok(s) => s,
        Err(e) => exit!("{}", e),
    };

    debug!("Reading class file");
    let class = match read_classfile::<File>(&mut file) {
        Ok(s) => s,
        Err(e) => exit!("{}", e),
    };

    if class.major_version > 52 {
        exit!("This JVM only supports class files upto version 52 (Java 1.8) but the given file has version {} (Java 1.{})", class.major_version, class.major_version - 44);
    }

    debug!(
        "Running class file version: {} (Java 1.{})",
        class.major_version,
        class.major_version - 44
    );
    let main = class.find_method("main", "([Ljava/lang/String;)V");
    debug!("Found method main(String[] args)");
    debug!("Starting main");

    let mut mframe = Frame::new(&main, &class.constant_pool, Option::None);
    mframe.exec();
    debug!("Main method ended");
    debug!("Shutting down VM. Goodbye!");
}
