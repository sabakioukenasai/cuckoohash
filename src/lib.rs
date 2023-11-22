extern crate ring;

mod bucket;
mod utils;

use std::iter::repeat;

use crate::utils::{get_double_hash, get_another_hash, get_random};

/// 最大重分配次数
pub const MAX_RELOCATE: usize = 256;

/// 默认哈希表大小
pub const DEFAULT_CAPACITY: usize = 1 << 12;

/// 默认元素大小
pub const DEFAULT_ELEM_BYTE_WIDTH: usize = 32;

pub const DEFAULT_STASH_SIZE: usize = 6;

define_bucket_bin!(22);

#[derive(Debug)]
struct Recorder{
	reload_elem: usize,
	reload_cnt: usize,
	max_reload: usize,
}

impl Recorder {
	fn new() -> Self {
		Self { reload_elem: 0, reload_cnt: 0, max_reload: 0 }
	}
}

#[derive(Debug)]
pub struct CuckooHashTable  {			// 随机数生成器
	buffer: Box<[Bucket]>,
	stash: Vec<Bin>,
	capacity: usize,
	len: usize,
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
			record: Recorder::new()
		}
	}

	/// 向Cuckoo Hash表中插入一个元素，如果两个hash位置中有任意一个是空闲的
	/// 就直接向该位置插入元素。如果两个位置都有元素，就随机选择一个作为牺牲者
	/// 然后将牺牲者插入到牺牲者的另一个哈希位置中去
	pub fn insert(&mut self, data: u32) -> bool {
		let mut bin = Bin::from_slice(&data.to_le_bytes());
		let (h1, h2) = get_double_hash(&bin.data, self.capacity);

		// 如果第一个 Bucket 有位置，就直接向其中插入元素
		// 否则就向第二个位置插入元素，如果第二个位置内原来就存在元素，则开始迭代替换
		if self.buffer[h1].is_available() {

			let _ = self.buffer[h1].insert(bin);
		} else if self.buffer[h2].is_available() {
			let _ = self.buffer[h2].insert(bin);
		} else {
			self.record.reload_elem += 1;
			let mut idx = if get_random() & 0x1 == 0 { h1 } else { h2 };

			for i in 0..MAX_RELOCATE {
				let victim = self.buffer[idx].insert(bin);
				if victim.is_none() {
					self.len += 1;
					return true;
				}
				bin = victim.unwrap();
				idx = get_another_hash(&bin.data, self.capacity, idx);
				self.record.reload_cnt += 1;
				self.record.max_reload = std::cmp::max(self.record.max_reload, i);
			}

			if self.buffer[idx].is_available() {
				let _ = self.buffer[idx].insert(bin);
			} else {
				self.stash.push(bin);
			}

		}

		self.len += 1;
		true
	}

	/// 判断一个元素是否在 Table 中
	pub fn contains(&self, data: u32) -> bool {
		let bin = Bin::from_slice(&data.to_le_bytes());
		let (h1, h2) = get_double_hash(&bin.data, self.capacity);
		self.buffer[h1].contains(&bin) || self.buffer[h2].contains(&bin) 
			|| self.stash.iter().any(|item| item == &bin)
	}

	pub fn stash_size(&self) -> usize {
		self.stash.len()
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use std::time::Instant;

	#[test]
	fn test_insert() {
		let f = (1 << 21) as f64 * 1.1;
		let mut ctable = CuckooHashTable::with_capacity(f as usize);

		let start = Instant::now();
		for i in 10000..(1u32 << 20) {
			ctable.insert(i);
		}

		let insert_time = start.elapsed().as_millis();
		println!("Insert Time: {}ms", insert_time);

		for i in 10000..(1u32 << 20) {
			assert!(ctable.contains(i));
		}
		println!("Check Time: {}ms", start.elapsed().as_millis() - insert_time);

		println!("table capacity: {}", ctable.capacity);
		println!("element count: {}", ctable.len);
		println!("use stash: {}", ctable.stash_size());
		println!("record: {:?}", &ctable.record);
	}
}