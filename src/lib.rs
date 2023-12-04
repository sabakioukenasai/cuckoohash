extern crate ring;

mod cuckoo_hash;
mod bucket;
mod utils;

pub use cuckoo_hash::standard::CuckooHashTable as StandardCuckoo;
pub use cuckoo_hash::threeway::CuckooHashTable as ThreewayCuckoo;