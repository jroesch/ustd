#![crate_name="ustd"]
#![crate_type = "rlib"]
#![feature(
    lang_items,
    no_std,
    libc,
    alloc,
    collections,
    unicode,
    raw,
    core_intrinsics
)]
#![no_std]

/// A tiny standard library for writing Rust inside of the jos kernel.
///
/// We attempt to bind and expose any Rust functionality that can be easily shipped
/// in the kernel.
///
/// We implement a couple helper modules for interfacing with the existing C implementation
/// as well as reimplementing some desirable functionality in `std` that works in the kernel.
extern crate alloc;
extern crate collections as core_collections;
extern crate libc;
extern crate rustc_unicode;

// Re-export all the core functionality just like `std`.
pub use core::any;
pub use core::cell;
pub use core::clone;
pub use core::cmp;
pub use core::convert;
pub use core::default;
pub use core::hash;
pub use core::intrinsics;
pub use core::iter;
pub use core::marker;
pub use core::mem;
pub use core::ops;
pub use core::ptr;
pub use core::raw;
#[allow(deprecated)]
// pub use core::simd;
pub use core::result;
pub use core::option;

pub use alloc::boxed;
pub use alloc::rc;

pub use core_collections::borrow;
pub use core_collections::fmt;
pub use core_collections::slice;
pub use core_collections::str;
pub use core_collections::string;
pub use core_collections::vec;

pub use rustc_unicode::char;

// Re-export common core types.
pub use core::isize;
pub use core::i8;
pub use core::i16;
pub use core::i32;
pub use core::i64;

pub use core::usize;
pub use core::u8;
pub use core::u16;
pub use core::u32;
pub use core::u64;

pub mod collections {
    pub use core_collections::*;
}

pub mod ffi;

/// This should ideally interface with the kernel memory system in order to call into the
/// memory manager if we run out of memory.
#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
