use core::mem::size_of;
use core::intrinsics::copy_memory;

pub struct IDTable {
  size: u16,
  addr: *mut IDTDescr
}

impl IDTable {
  pub fn new(cap: uint, table: *mut IDTDescr) -> IDTable {
    IDTable {
      size: (cap * size_of::<IDTDescr>()) as u16,
      addr: table
    }
  }

  // pub fn enable(&self, index: uint, itr: *mut IDTDescr) {
  //   let dst = self.addr + (index * size_of::<IDTDescr>());
  //   copy_memory(dst, itr, 1);
  // }
  
  #[inline]
  pub fn load(&self) {
    unsafe {
      asm!("lidt [$0]" :: "A"(self) :: "intel")
    }
  }
}

enum IDTFlags {
  INTR_GATE = 0b1110,
  TRAP_GATE = 0b1111,
  PRESENT   = 1 << 7
}


#[packed]
pub struct IDTDescr {
  offset_1: u16,
  selector: u16,
  zero: u8,
  type_attr: u8,
  offset_2: u16
}

impl IDTDescr {
  pub fn new(func: unsafe extern "C" fn(), sel: u16, flags: u8) -> IDTDescr {
    let addr = func as uint;
    let (addr_lo, addr_hi) = ( (addr & 0xFFFF0000) >> 16, (addr & 0x0000FFFF));
    IDTDescr {
      offset_1: addr_lo as u16,
      selector: sel,
      zero: 0,
      type_attr: flags,
      offset_2: addr_hi as u16
    }
  }
}