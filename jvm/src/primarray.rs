use crate::exit;
use crate::frame::RuntimeType;
pub enum ArrayType {
  BOOLEAN,
  CHAR,
  BYTE,
  SHORT,
  INT,
  LONG,
  FLOAT,
  DOUBLE
}

pub struct PrimArray {
  size: u32,
  ty: ArrayType,
  data: Vec<RuntimeType>
}

impl PrimArray {
  pub fn new(size: u32, ty: ArrayType) -> Self {
    PrimArray {
      size,
      ty,
      data: Vec::with_capacity(size as usize)
    }
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