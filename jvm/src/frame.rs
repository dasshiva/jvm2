use crate::javaclass::*;
use crate::log::debug;
use crate::exit;
use crate::primarray::{PrimArray, ArrayType};
use std::any::Any;

pub struct Frame<'a> {
    lvarray: Vec<RuntimeType>,
    opstack: OpStack,
    cp: &'a ConstantPool,
    code: Vec<u8>
}

impl<'a> Frame<'a> {
    pub fn new(info: &MethodInfo, cp: &'a ConstantPool, args: Option<RuntimeType> ) -> Self {
        let code_attr = match &info.attributes[0] {
            AttributeInfo::Code { max_stack, max_locals, code, exception_table, attributes } => (max_stack, max_locals, code),
            _ => exit!("Invalid attribute")
        };

        let mut lvarray: Vec<RuntimeType> = Vec::with_capacity(*code_attr.1 as usize);
        lvarray.push(RuntimeType::Boolean(0));
        let opstack = OpStack::new(*code_attr.0);
        let code = code_attr.2.clone();

        Frame {
            lvarray,
            opstack,
            cp,
            code
        }
    }

    pub fn exec(&mut self) {
        let mut pc = 0usize;
        println!("{}", self.code.len());
        while pc < 18 {
            match self.code[pc] {
                3 => { // iconst_0
                  self.opstack.push(Box::new(0i32));
                }
                
                4 => { // iconst_1
                  self.opstack.push(Box::new(1i32));
                }
                
                16 => { // bipush
                    pc += 1;
                    self.opstack.push(Box::new(self.code[pc] as i32));
                },

                27 => { // iload_1
                    let e = self.lvarray[1].clone();
                    match e {
                        RuntimeType::Int(s) => self.opstack.push(Box::new(s)),
                        _ => exit!("Value at index 1 of local variable array is not int")
                    };
                },
                
                45 => { // aload_3
                  let e = self.lvarray[3].clone();
                  match e {
                    RuntimeType::Reference(_, _) => self.opstack.push(Box::new(e)),
                    _ => exit!("Value at index 3 of local variable array is not a reference")
                  }
                },
                
                60 => { // istore_1
                  self.lvarray.insert(1, RuntimeType::Int(*self.opstack.pop().downcast_ref::<i32>().unwrap()));
                },

                61 => { // istore_2
                  self.lvarray.insert(2, RuntimeType::Int(*self.opstack.pop().downcast_ref::<i32>().unwrap()));
                },
                
                78 => { // astore_3
                  let elem = self.opstack.pop().downcast_ref::<RuntimeType>().unwrap().clone();
                  match elem {
                    RuntimeType::Reference(ref r, _) => {
                      let new = RuntimeType::Reference(*r, 3);
                      self.lvarray.insert(3, elem);
                    },
                    _ => exit!("Not a reference"),
                 }
                },
                
                85 => { // castore
                  let val = *self.opstack.pop().downcast_ref::<i32>().unwrap();
                  let index = *self.opstack.pop().downcast_ref::<i32>().unwrap();
                  let array = self.opstack.pop().downcast_ref::<RuntimeType>().unwrap().clone();
                  match array {
                    RuntimeType::Reference(obj, index) => match obj {
                      RefType::PArray(arr) => {
                       // arr.
                      },
                      _ => exit!("Reference is not a primitive type array"),
                    }
                    
                    _ => exit!("Not a reference"),
                  }
                },
                
                96 => { // iadd
                    let a = *self.opstack.pop().downcast_ref::<i32>().unwrap();
                    let b = *self.opstack.pop().downcast_ref::<i32>().unwrap();
                    self.opstack.push(Box::new(a + b));
                },
                
                188 => { // newarray
                  let size = *self.opstack.pop().downcast_ref::<i32>().unwrap();
                  pc += 1;
                  let t = self.code[pc];
                  match t {
                    5 => self.opstack.push(Box::new(RuntimeType::Reference(RefType::PArray(PrimArray::new(size as u32, ArrayType::CHAR)), 0))),
                    _ => exit!("Arrays for other primitive types are unimplemented ")
                  }
                }
                _ => exit!("Unimplemented instruction {}", self.code[pc])
            }

            pc += 1;
            println!("{pc}");
        }
    }
}

struct OpStack {
    size: u16,
    stack: Vec<Box<dyn Any>>,
    top: u16,
}

impl OpStack {
    pub fn new (lim: u16) -> Self {
        OpStack {
            size: lim,
            stack: Vec::with_capacity(lim as usize),
            top: 0
        }
    }

    pub fn push(&mut self, elem: Box<dyn Any>) {
        self.stack.push(elem);
        self.top += 1;
    }

    pub fn pop(&mut self) -> Box<dyn Any> {
        self.top -= 1;
        self.stack.remove(self.top as usize)
    }
}

#[derive(Clone)]
pub enum RuntimeType {
    Boolean(u8),
    Char(u16),
    Short(i16),
    Int(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Reference(RefType, u32),
    ReturnAddr(usize)
}

#[derive(Clone)]
pub enum RefType {
  PArray(PrimArray),
  Object,
}
