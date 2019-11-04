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

//extern { fn puts(s: *const u8); }
//extern "rust-intrinsic" { fn transmute<T, U>(t: T) -> U; }

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "eh_unwind_resume"] extern fn eh_unwind_resume() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
#[no_mangle] pub extern fn rust_eh_register_frames () {}
#[no_mangle] pub extern fn rust_eh_unregister_frames () {}
