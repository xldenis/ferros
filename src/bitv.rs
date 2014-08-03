use core::mem::transmute;

pub struct Bitv {
  pub storage: *mut u32
}


impl Bitv {
  pub fn get(&self, pos u8) -> u8 {
    let w = pos / 16;
    let o = (pos % 16) * 2;
    unsafe {
      transmute((*self.storage.offset(w) as uint >> b) as u8 & 3)
    }
  }

  pub fn set(&self, pos: u8, val: u8) {
    let w = pos / 16;
    let o = (pos % 16) * 2; 
    unsafe {
      *self.storage.offset(w) = *self.storage.offset(w) & !(3 << o) | (val << o);
    } 
  }

  pub fn clear(&self, cap: uint) {
    unsafe {
      set_memory(self as *mut u8, 0, cap / 4);
    }
  }
}