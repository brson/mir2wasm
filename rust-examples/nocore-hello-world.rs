#![feature(intrinsics, lang_items, start, no_core, fundamental)]
#![no_core]

//! This is a minimal libcore-like library that can be used by tests
//! before we support the actual libcore.

#[lang = "sized"]
#[fundamental]
pub trait Sized { }

#[lang = "copy"]
pub trait Copy : Clone { }

#[lang = "freeze"]
unsafe trait Freeze {}

pub trait Clone : Sized { }

#[lang = "add"]
pub trait Add<RHS = Self> {
    type Output;
    fn add(self, rhs: RHS) -> Self::Output;
}

impl Add for isize {
    type Output = isize;
    fn add(self, rhs: isize) -> Self::Output { self + rhs }
}

#[link(name = "c")]
extern { }

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "eh_unwind_resume"]
extern fn eh_unwind_resume() {}

#[cold]
#[inline(never)]
#[lang = "panic"]
pub fn panic(_expr_file_line: &(&'static str, &'static str, u32)) -> ! {
    loop {}
}

#[no_mangle] pub extern fn rust_eh_register_frames () {}
#[no_mangle] pub extern fn rust_eh_unregister_frames () {}

// access to the wasm "spectest" module test printing functions
mod wasm {
    pub fn print_i32(i: isize) {
        unsafe { _print_i32(i); }
    }

    extern {
        fn _print_i32(i: isize);
    }
}

fn real_main() -> isize {
    let i = 1;
    let j = i + 2;
    j
}

#[start]
fn main(_: isize, _: *const *const u8) -> isize {
    /*unsafe {
        let (ptr, _): (*const u8, usize) = transmute("Hello!\0");
        puts(ptr);
}*/

    let result = real_main() + 3;
    wasm::print_i32(result); // (i32.const 6)
    result
}
