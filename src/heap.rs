use util::bitv::Bitv;
use mem::{Alloc, BuddyAlloc, Allocator};
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
pub unsafe fn alloc(size: uint, align: uint) -> *mut u8 {
  match expect(heap).alloc(size) {
  (_, 0) =>  out_of_memory(),
  (ptr,_) => ptr
  }
}

#[no_mangle]
pub unsafe extern "C" fn rust_allocate(size: uint, _align: uint) -> *mut u8 {
  alloc(size, 0)
}

#[inline]
#[lang="exchange_free"]
pub unsafe fn free(ptr: *mut u8, _size: uint, _align: uint) {
  expect(heap).free(ptr as *mut u8);
}

#[inline]
pub unsafe fn realloc(ptr: *mut u8, size: uint) -> *mut u8 {
  match expect(heap).realloc(ptr,size) {
    (_, 0) =>  out_of_memory(),
    (ptr,_) => ptr 
  }
}

fn expect<T>(opt : Option<T>) -> T {
    match opt {
        Some(val) => val,
        None => unsafe {abort()}
    }
}

#[inline]
fn out_of_memory() -> ! {
  unsafe {abort()}
}