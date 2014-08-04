use core::ptr::RawPtr;
use core::mem::transmute;
use core::ptr::{set_memory};

pub struct Bitv {
  pub storage: *mut u32
}


impl Bitv {
  pub fn get(&self, pos: uint) -> u8 {
    let w = (pos / 16) as int;
    let o = (pos % 16) * 2;
    unsafe {
      transmute((*self.storage.offset(w) as uint >> o) as u8 & 3)
    }
  }

  pub fn set(&self, pos: uint, val: u8) {
    let w = (pos / 16) as int;
    let o = (pos % 16) * 2; 
    unsafe {
      *self.storage.offset(w) = *self.storage.offset(w) & !(3 << o) | (val as u32 << o);
    } 
  }

  #[inline]
  fn as_mut_ptr(&self) -> *mut u8 {
    self.storage as *mut u8
  }

  pub fn clear(&self, cap: uint) {
    unsafe {
      set_memory(self.as_mut_ptr() , 0, cap / 4);
    }
  }
}