#![feature(no_std, lang_items)]
#![no_std]

extern crate rlibc;

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
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! {loop{}}
