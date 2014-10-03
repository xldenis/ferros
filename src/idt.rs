use core::mem::size_of;
use core::mem::transmute;
use core::ptr::RawPtr;
use heap;

pub struct IDTReg {
  size: u16,
  addr: *mut IDTDescr
}

impl IDTReg {
  pub fn new(table: *mut IDTDescr, size: u16) -> IDTReg{
    IDTReg {
      size: size,
      addr: table
    }
  }
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

pub struct IDTable {
  reg: &'static IDTReg,
  table: *mut IDTDescr
}

impl IDTable {
  pub fn new() -> IDTable{
    unsafe {
      let table = heap::alloc(size_of::<IDTDescr>() * 256,0) as *mut IDTDescr;
      let reg   = heap::alloc(size_of::<IDTDescr>(),0) as *mut IDTReg;
      *(reg as *mut IDTReg) = IDTReg::new(table, 256);

      IDTable {
        reg: transmute(reg),
        table: table
      }
    }
  }
}