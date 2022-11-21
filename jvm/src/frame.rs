use crate::javaclass::*;
use crate::log::debug;
use crate::exit;
use std::any::Any;

pub struct Frame<'a> {
    lvarray: Vec<LocalVarType>,
    opstack: OpStack,
    cp: &'a ConstantPool,
    code: Vec<u8>
}

impl<'a> Frame<'a> {
    pub fn new(info: &MethodInfo, cp: &'a ConstantPool) -> Self {
        let code_attr = match &info.attributes[0] {
            AttributeInfo::Code { max_stack, max_locals, code, exception_table, attributes } => (max_stack, max_locals, code),
            _ => exit!("Invalid attribute")
        };

        let lvarray: Vec<LocalVarType> = Vec::with_capacity(*code_attr.1 as usize);
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
        while pc < self.code.len() {
            match self.code[pc] {
                16 => { // bipush
                    pc += 1;
                    self.opstack.push(Box::new(self.code[pc] as i32));
                },

                27 => { // iload_1
                    let e = self.lvarray[0];
                    match e {
                        LocalVarType::Int(s) => self.opstack.push(Box::new(s)),
                        _ => exit!("Value at index 1 of local variable array is not int")
                    };
                },
                60 => { // istore_1
                    self.lvarray.push(LocalVarType::Int(*self.opstack.pop().downcast_ref::<i32>().unwrap()));
                },

                61 => return, // return 

                96 => { // iadd
                    let a = *self.opstack.pop().downcast_ref::<i32>().unwrap();
                    let b = *self.opstack.pop().downcast_ref::<i32>().unwrap();
                    self.opstack.push(Box::new(a + b));
                },
                _ => exit!("Unimplemented instruction {}", self.code[pc])
            }
            self.opstack.show_state();

            pc += 1;
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

    pub fn show_state(&self) {
        debug!("Stack dump");
        let mut i = 0u16;
        while i < self.top {
            println!("{}",self.stack[i as usize].downcast_ref::<i32>().unwrap());
            i += 1;
        }
    }
}

#[derive(Copy, Clone)]
enum LocalVarType {
    Boolean(u8),
    Char(u16),
    Short(i16),
    Int(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Reference,
    ReturnAddr(usize)
}
