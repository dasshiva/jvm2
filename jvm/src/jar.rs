use crate::zip::read;
use crate::javaclass::*;
use crate::exit;
//use crate::log::*;
use std::env;
use std::ffi::OsString;

pub struct ClassLoader {
    name: String,
    class: javaclass::ClassFile
}

impl ClassLoader {
    pub fn load_class(class_name: &str)  {
        let cp = match env::var_os("CLASSPATH") {
            Some(s) => s.to_os_string(),
            None => OsString::new()
        }.into_string().unwrap();
    }
}
