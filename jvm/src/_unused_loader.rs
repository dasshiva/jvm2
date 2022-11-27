// This file has been abandoned.
use crate::zip::read;
use std::path::Path;
use crate::javaclass::*;
use crate::exit;
//use crate::log::*;
use std::env;
use std::ffi::OsString;

pub struct ClassLoader {
    name: String,
    class: javaclass::ClassFile
}

fn split(path: &String) -> Vec<String> {
  let mut ret: Vec<String> = Vec::new();
  let mut acc = String::new();
  for i in path.chars() {
    if i == ':' {
      if acc.len() == 0 { continue; }
      ret.push(acc.clone());
      acc.clear();
    }
    acc.push(i);
  }
  ret
}

impl ClassLoader {
    pub fn load_class(class_name: &str)  {
        let mut cp = match env::var_os("CLASSPATH") {
            Some(s) => s.to_os_string(),
            None => OsString::new()
        }.into_string().unwrap();
        cp.push(':');
        if cp.len() != 0 {
          let paths = split(&cp);
          for path in paths.iter() {
            let p = Path::new(&path);
            if p.metadata().unwrap().is_file() {
              if 
            }
          }
        }
    }
}
