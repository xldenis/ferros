use core::ptr::RawPtr;
use core::mem::transmute;
use core::ptr::{copy_memory};
use core::intrinsics::{ctlz32, offset};
use core::intrinsics::offset;
use util::bitv::Bitv;

enum Node {
  USED   = 0,
  UNUSED = 1,
  FREE   = 2,
  SPLIT  = 3,
  FULL   = 4
} 

pub trait Allocator {
  fn alloc(&mut self, size: uint) -> (*mut u8, uint);

  fn realloc(&mut self, src: *mut u8, s: uint) -> (*mut u8, uint) {
    self.free(src);
    let(ptr, size) = self.alloc(s);
    unsafe { copy_memory(ptr, src as *const u8, s); }
    (ptr, size)
  }

  fn free(&mut self, ptr: *mut u8);
}

pub struct BuddyAlloc {
  pub order: uint,
  pub tree: Bitv
}

pub struct Alloc {
  pub buddy: BuddyAlloc,
  pub base: *mut u8,
  pub el_size: uint
}

impl BuddyAlloc {
  pub fn new(order: uint, storage: Bitv) -> BuddyAlloc {
    storage.clear(1 << (order + 1));
    BuddyAlloc {order: order, tree: storage}
  }
  fn alloc(&mut self, mut size: uint) -> (uint, uint) {
    size = 32 - unsafe { ctlz32(size as u32 - 1) } as uint;

    let mut level = self.order;
    let mut index = 0;
    loop {
      match(self.get(index), level == size) {

        (UNUSED, true) => {
          self.set(index, USED);
          let mut parent = index;
          loop {
            let buddy = parent - 1 + (parent & 1) * 2;
            match self.get(buddy) {
              USED | FULL if parent > 0 => {
                parent = (parent + 1) / 2 - 1;
                self.set(parent, FULL);
              }
              _ => break
            }
          }
          return (
            self.offset(index, level),
            1 << size
            );
        }
        (UNUSED, false) => {
          // this node is large and unused, split it
          self.set(index, SPLIT);
          self.set(2*index+1, UNUSED);
          self.set(2*index+2, UNUSED);
          level -= 1;
          index = 2*index+1;
        }
        (SPLIT, false) => {
          // too large but allocated follow it down
          level -= 1;
          index = 2*index;
        }
        _ => loop {
          // left child
          if index & 1 == 1 {
            index += 1;
            break;
          }
          //up
          level += 1;

          if index == 0 {
            return (0, 0);
          }
          //move ptr up
          index = (index + 1) / 2 - 1;
        }
      }
    }
  }

  fn free(&mut self, offset: uint) {
    let mut length = 1 << self.order;
    let mut left = 0;
    let mut index = 0;

    loop {
      match self.get(index) {
        UNUSED => return,
        USED => loop {
          if index == 0 {
            self.set(0, UNUSED)
          }

          let buddy = index - 1 + (index & 1) * 2;
          match self.get(buddy) {
            UNUSED => {},
            _ => {
              self.set(index, UNUSED);
              loop {
                let parent = (index + 1) / 2 - 1;
                match self.get(parent) {
                  FULL if index > 0 => {
                    self.set(parent, SPLIT)
                  },
                  _ => return
                }
                index = parent
              }
            }
          }
          index = (index + 1) / 2 - 1; 
        },
        _ => {
          length /= 2;

          if offset < left + length {
            // go left
            index = index * 2 + 1;
          } else {
            // go right
            index = index * 2 + 2;
            left += length; 
          }

        }
      }
    }
  }
  #[inline]
  fn offset(&self, index: uint, level: uint) -> uint {
    (index + 1 - (1 << self.order >> level)) << level
  }

  fn get(&self, i: uint) -> Node {
    unsafe {
      transmute(self.tree.get(i))
    }
  }

  fn set(&self, i: uint, v: Node) {
    self.tree.set(i, v as u8);
  }
}
impl Alloc {
  pub fn new(buddy: BuddyAlloc, base: *mut u8, size: uint) -> Alloc {
    return Alloc { buddy: buddy, base: base, el_size: size}
  }
}

impl Allocator for Alloc {
  fn alloc(&mut self, size: uint) -> (*mut u8, uint) {
    let (pos, size) = self.buddy.alloc(size);
    return ( 
      unsafe {self.base.offset((pos << self.el_size) as int )}, 
      size << self.el_size
      )
  }
  fn free(&mut self, ptr: *mut u8) {
    let len = 1 << self.buddy.order << self.el_size;
    unsafe {
      if ptr < self.base || ptr > self.base.offset(len) {  return; } // oob
    }
    let offset = (ptr as uint - self.base as uint) >> self.el_size;
    self.buddy.free(offset)
  }
}
