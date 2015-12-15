#![feature(lang_items, const_fn, unique, core_str_ext)]
#![no_std]

extern crate rlibc;
mod writer;

use writer::Color;
use core::ptr::Unique;
use core::fmt::Write;
// Implementation of color clear taken mostly from charliesome / rustboot

fn range<F>(low: usize, high: usize, iter: F)
  where F : Fn(usize) -> () {

  let mut cur = low;
  while cur < high {
    iter(cur);
    cur += 1;
  }
}

fn clear_screen(background: Color) {
  let shifted_bg = (background as u16) << 12;
  range(0, 80*25, |i| {
    unsafe {
    *((0xb8000 + i * 2) as *mut u16) = shifted_bg;
    }
  });
}

#[no_mangle]
pub fn main() {
  clear_screen(Color::LightPink);
  let mut writer = writer::Writer {
    column_pos: 0,
    color: writer::ColorCode::new(Color::White, Color::Black),
    buffer: unsafe {Unique::new(0xb8000 as *mut _)},
  };
  writer.write_str("Hello World!");
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! {loop{}}
