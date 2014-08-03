#![crate_name = "main"]
#![crate_type = "staticlib"]
#![no_std]
#![feature(asm, macro_rules, default_type_params, phase, globs, lang_items, intrinsics)]

#[phase(plugin, link)]
extern crate core;

enum Color {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Pink       = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    LightPink  = 13,
    Yellow     = 14,
    White      = 15,
}

// Implementation of color clear taken mostly from charliesome / rustboot

fn range(low: uint, high: uint, iter: |uint| -> ()) {
  let mut cur = low;
  while cur < high {
    iter(cur);
    cur += 1;
  }
}

fn clear_screen(background: Color) {
  range(0, 80*25, |i| {
    unsafe {
    *((0xb8000 + i * 2) as *mut u16) = (background as u16) << 12;
    }
  });
}

#[no_mangle]
#[no_split_stack]
pub fn main() {
  clear_screen(LightRed);
}
