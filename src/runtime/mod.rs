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

#[no_mangle]
pub fn memcpy(dest: *mut u8, src: *const u8, mut n: uint) {
    if unlikely!(n == 0) {
        return;
    }
    unsafe {
        if n < 12 {
            asm!("rep movsb" :: "{edi}"(dest), "{esi}"(src), "{ecx}"(n))
            return;
        }

        let offset = (4 - (dest as uint % 4)) % 4;
        n -= offset;

        let mut pd: *mut u8;
        let mut ps: *const u8;
        asm!("rep movsb" : "={edi}"(pd), "={esi}"(ps) : "{edi}"(dest), "{esi}"(src), "{ecx}"(offset))
        asm!("rep movsl" : "={edi}"(pd), "={esi}"(ps) : "{edi}"(pd), "{esi}"(ps), "{ecx}"(n >> 2))
        asm!("rep movsb" :: "{edi}"(pd), "{esi}"(ps), "{ecx}"(n % 4))
    }
}

#[no_mangle]
pub fn memmove(dest: *mut u8, src: *const u8, n: uint) {
    unsafe {
        if src < dest as *const u8 {
            asm!("std")
            memcpy(dest.offset(n as int), src.offset(n as int), n);
            asm!("cld")
        }
        else {
            asm!("cld")
            memcpy(dest, src, n);
        }
    }
}

#[no_mangle]
pub unsafe fn memcmp(s1: *const u8, s2: *const u8, n: uint) -> i32 {
    let mut i = 0;
    while i < n {
        let a = *s1.offset(i as int);
        let b = *s2.offset(i as int);
        if a != b {
            return (a - b) as i32
        }
        i += 1;
    }
    return 0;
}
