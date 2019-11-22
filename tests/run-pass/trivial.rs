#![feature(custom_attribute, fundamental, lang_items, no_core)]
#![allow(dead_code, unused_attributes)]

#![no_std]
#![no_core]

pub mod tinycore;

fn empty() {}

fn unit_var() {
    let x = ();
    x
}

fn main() {}
