#![no_std]
#![feature(c_variadic)]
#![feature(raw_vec_internals)]
#![feature(const_btree_new)]
#![feature(const_if_match)]
#![feature(llvm_asm)]

extern crate alloc;

pub mod binary;
pub mod compatibility;
pub mod wifi;

mod log;
