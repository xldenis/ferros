#![crate_name = "main"]
#![feature(asm, macro_rules, default_type_params, phase, globs, lang_items, intrinsics)]
pub mod mem;


fn main() {
  let x: *mut u8 = (2<<16);
}