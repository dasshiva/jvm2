use crate::javaclass::ClassFile;
use crate::javaclass::read_classfile;
use std::fs::File;
use std::path::PathBuf;
use crate::exit;
/*
NOTE: This file implements a very basic boot class loader
Support for jars is not yet present and it can only load raw class files
All such class files must be present in one directory.
This directory can be specified at the command line. If unspecified it will assume the directory to be "./lib" (".\lib" on windows)

NOTE: This jvm does not use the jdk standard platform library. Instead it uses its own implementation of the standard library which you will find in this repository under /lib
*/

#[cfg(not(target_os = "windows"))]
static CLASSPATH: &str = "./lib/";

#[cfg(target_os = "windows")]
static CLASSPATH: &str = r".\lib\";

struct Loader (Option<String>);

impl Loader {
  fn load_class_default(&self, src: &String) -> ClassFile {
    let mut path = PathBuf::from(CLASSPATH);
    path.push(src);
    
    let mut file = match File::open(path.as_path()) {
       Ok(s) => s,
       Err(e) => exit!("{}", e)
    };
    
    match read_classfile::<File>(&mut file) {
      Ok(s) => s,
      Err(e) => exit!("{}", e)
    }
  }
  
  pub fn load_class(&self, src: &String) -> ClassFile {
    if self.0.is_some() {
      let mut path = PathBuf::from(&self.0.as_ref().unwrap());
      path.push(src);
      return match File::open(path.as_path()) {
        Ok(mut s) => match read_classfile(&mut s) {
          Ok(s) => s,
          Err(e) => exit!("{}", e)
        }
        Err(e) => self.load_class_default(src)
      }
    }
    
    self.load_class_default(src)
  }
  
}