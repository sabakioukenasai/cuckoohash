extern crate ring;

mod bucket;
mod utils;

use std::iter::repeat;

use crate::utils::{get_double_hash, get_another_hash, get_random};
use crate::bucket::{Bin, Bucket};

/// 最大重分配次数
pub const MAX_RELOCATE: usize = 256;

/// 默认哈希表大小
pub const DEFAULT_CAPACITY: usize = 1 << 12;

/// 默认元素大小
pub const DEFAULT_ELEM_BYTE_WIDTH: usize = 32;

pub const DEFAULT_STASH_SIZE: usize = 6;

#[cfg(test)]
#[derive(Debug)]
struct Recorder{
	reload_elem: usize,
	reload_cnt: usize,
	max_reload: usize,
}

#[cfg(test)]
impl Recorder {
	fn new() -> Self {
		Self { reload_elem: 0, reload_cnt: 0, max_reload: 0 }
	}
}

#[derive(Debug)]
pub struct CuckooHashTable{
	buffer: Box<[Bucket]>,
	stash: Vec<Bin>,
	capacity: usize,
	len: usize,
	#[cfg(test)]
	record: Recorder,
}

impl CuckooHashTable
{
	/// 创建一个能容纳cap个元素的的Cuckoo Hash Table
	pub fn with_capacity(cap: usize) -> Self {
		let capacity = std::cmp::max(1, cap);
		let stash = Vec::with_capacity(DEFAULT_STASH_SIZE);

		Self {
			buffer: repeat(Bucket::new())
				.take(capacity)
				.collect::<Vec<_>>()
				.into_boxed_slice(),
			len: 0,
			capacity,
			stash,
			#[cfg(test)]
			record: Recorder::new()
		}
	}

	/// 向Cuckoo Hash表中插入一个元素，如果两个hash位置中有任意一个是空闲的
	/// 就直接向该位置插入元素。如果两个位置都有元素，就随机选择一个作为牺牲者
	/// 然后将牺牲者插入到牺牲者的另一个哈希位置中去
	pub fn insert(&mut self, data: u32) -> bool {
		let mut bin = Bin::from_slice(&data.to_le_bytes());
		let (h1, h2) = get_double_hash(bin.as_ref(), self.capacity);

		// try to put `data` into an empty `Bucket` if there exists.
		if self.try_put(&mut bin, h1) || self.try_put(&mut bin, h2) {
			self.len += 1; 
			return true;
		}

		#[cfg(test)] {
			self.record.reload_elem += 1;
		}

		let mut idx = if get_random(1) == 0 { h1 } else { h2 };

		for _i in 0..MAX_RELOCATE {
			self.buffer[idx].insert(&mut bin);
			if bin.is_empty() {
				self.len += 1;
				return true;
			}
			idx = get_another_hash(bin.as_ref(), self.capacity, idx);
			
			#[cfg(test)] {
				self.record.reload_cnt += 1;
				self.record.max_reload = std::cmp::max(self.record.max_reload, _i);
			}
		}

		if self.buffer[idx].is_available() {
			self.buffer[idx].insert(&mut bin);
		} else {
			self.stash.push(bin);
		}

		self.len += 1;
		true
	}

	/// 判断一个元素是否在 Table 中
	pub fn contains(&self, data: u32) -> bool {
		let bin = Bin::from_slice(&data.to_le_bytes());
		let (h1, h2) = get_double_hash(bin.as_ref(), self.capacity);

		self.buffer[h1].contains(&bin) || 
		self.buffer[h2].contains(&bin) || 
		self.stash.iter().any(|item| item == &bin)
	}

	pub fn stash_size(&self) -> usize {
		self.stash.len()
	}

	fn try_put(&mut self, bin: &mut Bin, idx: usize) -> bool {
		if self.buffer[idx].is_available() {
			self.buffer[idx].insert(bin);
			true
		} else {
			false
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use std::time::Instant;

	#[test]
	fn test_insert() {
		let f = (1 << 21) as f64 * 1.2;
		let mut ctable = CuckooHashTable::with_capacity(f as usize);

		let start = Instant::now();
		for i in 10000..(1u32 << 20) + 10000 {
			ctable.insert(i);
		}

		let insert_time = start.elapsed().as_millis();
		println!("Insert Time: {}ms", insert_time);

		for i in 10000..(1u32 << 20) + 10000 {
			assert!(ctable.contains(i));
		}
		println!("Check Time: {}ms", start.elapsed().as_millis() - insert_time);

		println!("table capacity: {}", ctable.capacity);
		println!("element count: {}", ctable.len);
		println!("use stash: {}", ctable.stash_size());
		println!("record: {:?}", &ctable.record);
	}
}