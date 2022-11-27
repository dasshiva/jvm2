use crate::exit;
use crate::frame::RuntimeType;

#[derive(Copy, Clone)]
pub enum ArrayType {
    BOOLEAN,
    CHAR,
    BYTE,
    SHORT,
    INT,
    LONG,
    FLOAT,
    DOUBLE,
}

#[derive(Clone)]
pub struct PrimArray {
    size: u32,
    ty: ArrayType,
    data: Vec<RuntimeType>,
}

impl PrimArray {
    pub fn new(size: u32, ty: ArrayType) -> Self {
        let mut ins = PrimArray {
            size,
            ty,
            data: Vec::with_capacity(size as usize),
        };

        unsafe {
            ins.data.set_len(size as usize);
        }
        ins
    }

    pub fn set(&mut self, index: usize, data: RuntimeType) {
        if index >= self.size as usize {
            exit!("java.lang.ArrayIndexOutOfBoundsException: Index {} is invalid for array of size {}",index, self.size);
        }
        self.data.insert(index, data);
    }

    pub fn get(self, index: usize) -> RuntimeType {
        if index >= self.size as usize {
            exit!("java.lang.ArrayIndexOutOfBoundsException: Index {} is invalid for array of size {}",index, self.size);
        }

        self.data[index].clone()
    }
}
