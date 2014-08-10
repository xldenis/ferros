use util::bitv::Bitv;
use mem::{Alloc, BuddyAlloc};
use core::prelude::*;
use core::intrinsics::abort;

pub static mut heap: Option<Alloc> = None;

pub fn init() -> Alloc {
  let alloc = Alloc::new(
    BuddyAlloc::new( 17, Bitv{storage: 0x100_000 as *mut u32 }),
    0x110_000 as *mut u8,
    0);
  unsafe {
    heap = Some(alloc);
  }
  alloc
}
#[inline]
#[lang="exchange_malloc"]
pub unsafe fn alloc(size: uint) -> *mut u8 {
  match expect(heap).alloc(size) {
  (_, 0) =>  0,
  (ptr,_) => ptr
  }
}
#[inline]
#[lang="exchange_free"]
pub unsafe fn free(ptr: *mut u8) {
  expect(heap).free(ptr)
}

pub unsafe fn realloc(ptr: *mut u8, size: uint) -> *mut u8 {

}

fn expect<T>(opt : Option<T>) -> T {
    match opt {
        Some(val) => val,
        None => abort(),
    }
}