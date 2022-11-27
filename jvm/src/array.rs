use crate::exit;
use crate::frame::RuntimeType;

pub struct Array {
  size: u32,
  ty: u8,
  data: Vec<RuntimeType>
}

impl Array {
  pub fn new(size: u32, ty: u8) -> Self {
    Array {
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