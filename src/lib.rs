#![feature(
    custom_attribute,
    link_args,
    rustc_private,
)]

// FIXME: C++ static linkage hacks. How do you do this for real?!
#[link_args = "-lstdc++ -static-libstdc++"]
extern "C" {}

#[macro_use]
extern crate rustc;
extern crate rustc_mir;
extern crate syntax;
extern crate rustc_const_math;
extern crate rustc_data_structures;

extern crate libc;

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate env_logger;

pub mod error;
pub mod trans;
mod binaryen;
mod binops;
mod monomorphize;
mod traits;
