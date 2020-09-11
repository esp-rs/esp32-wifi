#![no_std]
#![feature(c_variadic)]
#![feature(raw_vec_internals)]
#![feature(const_btree_new)]
#![feature(const_if_match)]
#![feature(llvm_asm)]
#![feature(associated_type_bounds)]
#![feature(const_fn)]
#![feature(binary_heap_retain)]

extern crate alloc;

pub mod binary;
pub mod compatibility;
pub mod timer;
pub mod wifi;

mod log;
