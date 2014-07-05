#![no_std]
#![allow(ctypes)]

// Implementation of color clear taken mostly from charliesome / rustboot

fn range(low: uint, high: uint, iter: |uint| -> ()) {
  let mut cur = low;
  while cur < high {
    iter(cur);
    cur++;
  }
}

fn clear_screen(background: Color) {
  range(0, 80*25, |i| {
    unsafe {
    *((0xb80000 + i * 2) as *mut u16) = (background as u16) << 12;
    }
  }
}

#[no_mangle]
#[no_split_stack]
pub fn main() {
  clear_screen(5);
}