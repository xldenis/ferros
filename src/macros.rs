// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern {
    #[link_name = "llvm.expect.i8"]
    pub fn u8_expect(val: u8, expected_val: u8) -> u8;
}

macro_rules! likely(
    ($val:expr) => {
        {
            let x: bool = $val;
            unsafe { ::macros::u8_expect(x as u8, 1) != 0 }
        }
    }
)

macro_rules! unlikely(
    ($val:expr) => {
        {
            let x: bool = $val;
            unsafe { ::macros::u8_expect(x as u8, 0) != 0 }
        }
    }
)