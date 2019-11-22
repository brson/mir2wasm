//! This is a minimal libcore-like library that can be used by tests
//! before we support the actual libcore.

#[lang = "sized"]
#[fundamental]
pub trait Sized { }

#[lang = "copy"]
pub trait Copy : Clone { }

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

#[lang = "not"]
pub trait Not {
    type Output;
    fn not(self) -> Self::Output;
}

impl Not for bool {
    type Output = bool;
    fn not(self) -> Self::Output { !self }
}

#[lang = "eq"]
pub trait PartialEq<Rhs: ?Sized = Self> {
    fn eq(&self, other: &Rhs) -> bool;

    fn ne(&self, other: &Rhs) -> bool { !self.eq(other) }
}

impl PartialEq<i16> for i16 {
    fn eq(&self, other: &i16) -> bool { *self == *other }

}

impl PartialEq<i64> for i64 {
    fn eq(&self, other: &i64) -> bool { *self == *other }

}

// #[link(name = "c")]
// extern { }

//extern { fn puts(s: *const u8); }
//extern "rust-intrinsic" { fn transmute<T, U>(t: T) -> U; }

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "eh_unwind_resume"] extern fn eh_unwind_resume() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
#[no_mangle] pub extern fn rust_eh_register_frames () {}
#[no_mangle] pub extern fn rust_eh_unregister_frames () {}

#[link(name = "rustrt")]
extern {
    pub fn panic() -> !;
}

macro_rules! panic {
    () => (
        panic!("explicit panic")
    );
    ($msg:expr) => (unsafe {
        $crate::tinycore::panic()
    });
}

macro_rules! assert {
    ($cond:expr) => (
        if !$cond {
            panic!(concat!("assertion failed: ", stringify!($cond)))
        }
    );
}

macro_rules! assert_eq {
    ($value:expr, $expected:expr) => {
        assert!(($value) == ($expected));
    }
}
