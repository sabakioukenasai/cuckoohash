//! As it's the faster scheme to perform PSI(Priate Set Intersection),
//! we aim to provide `Rust` version of cuckoo table as a widget of PSI.
//! 
//! In these project, we implement `Standard Cuckoo Table`(using two 
//! hashing funciton) and `Three-way Cuckoo Table`(using three hashing
//! funciton).
//! 
//! As the widget of PSI protocol, duplicate elements are not considered
//! in our implementation, other words, inserting duplicate element into
//! the cuckoo table will always false.
//! 
//! We use `Rust` crate `ring` to provide the hash funciton and
//! randomness generator. Note that our implementation is intended 
//! for learning purposes only and has not been subjected to 
//! rigorous security checks and is not intended for use in development.

extern crate ring;

mod cuckoo_hash;
mod bucket;
mod utils;

pub use cuckoo_hash::standard::CuckooHashTable as StandardCuckoo;
pub use cuckoo_hash::threeway::CuckooHashTable as ThreewayCuckoo;