use core::ptr::RawPtr;

#[inline]
fn stosb(s: *mut u8, c: u8, n: uint) {
  unsafe {
    asm!("rep stosb" :: "{al}"(c), "{edi}"(s), "{ecx}"(n))
  }
}
#[inline]
fn stosd8(s: *mut u8, c: u8, n: uint) {
  unsafe {
    let mut dword: u32 = c as u32;
    dword |= (dword << 24) | (dword << 16) | (dword << 8);
    asm!("rep stosl" :: "A"(dword), "{edi}"(s), "{ecx}"(n))
  }
}

#[inline]
fn stosd16(s: *mut u8, c: u16, n: uint) {
  unsafe {
    let mut dword: u32 = c as u32;
    dword |= dword << 16;
    asm!("rep stosl" :: "A"(dword), "{edi}"(s), "{ecx}"(n))
  }
}

#[inline]
fn stosd(s: *mut u8, c: u32, n: uint) {
    unsafe {
        asm!("rep stosl" :: "A"(c), "{edi}"(s), "{ecx}"(n))
    }
}

#[inline]
fn bmemset(mut s: *mut u8, c: u8, mut n: uint) {
  if unlikely!(n == 0) {
    return
  }
  if unlikely!(n == 1) {
    unsafe {*s = c; }
    return
  }

  while n > 0 {
    match n % 4 {
      0 => {
        stosd8(s, c, n / 4);
        n = 0;
      }
      q => {
        stosb(s, c, q);
        s = unsafe { s.offset(q as int) };
        n -= q; 
      }
    }
  }
}

pub fn wmemset(mut s: *mut u8, c: u16, n: uint) {
  if unlikely!(n == 0) {
    return 
  }
  if (n % 2) == 1 {
    unsafe {
      *(s as *mut u16) = c;
        s = s.offset(2);
    }
  }

  stosd16(s, c, n >> 1);
}

pub fn dmemset(s: *mut u8, c: u32, n: uint) {
  if unlikely!(n == 0) {
    return
  }

  stosd(s, c, n);
}

#[no_mangle]
pub fn memset(s: *mut u8, c: i32, n: int) {
  bmemset(s, (c & 0xFF) as u8, n as uint);
}
