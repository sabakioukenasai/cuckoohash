use crate::utils::*;
use crate::bucket::*;
use super::*;

use std::iter::repeat;

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
			capacity,
			stash,
			len: 0,
			#[cfg(test)]
			record: Recorder::new()
		}
	}

	/// 向Cuckoo Hash表中插入一个元素，如果两个hash位置中有任意一个是空闲的
	/// 就直接向该位置插入元素。如果两个位置都有元素，就随机选择一个作为牺牲者
	/// 然后将牺牲者插入到牺牲者的另一个哈希位置中去
	pub fn insert(&mut self, data: u32) -> bool {
		let mut bin = Bin::from_slice(&data.to_le_bytes());
		let hset = get_two_hash(bin.as_ref(), self.capacity);
		
		if self.try_put(&mut bin, hset.0) || self.try_put(&mut bin, hset.1) {
			self.len += 1;
			return true;
		}

		// recording number of elements have been reloaded.
		#[cfg(test)] {
			self.record.reload_elem += 1;
		}

		// choose a victim for reloading.
		let mut victim = if get_random(2) == 0 {
			hset.0
		} else {
			hset.1
		};

		for _i in 0..MAX_RELOCATE {
			self.buffer[victim].insert(&mut bin);
			if bin.is_empty() {
				self.len += 1;
				return true;
			}
			victim = get_alt_hash(bin.as_ref(), self.capacity, victim);
			
			#[cfg(test)] {
				self.record.reload_cnt += 1;
				self.record.max_reload = std::cmp::max(self.record.max_reload, _i);
			}
		}

		if self.buffer[victim].is_empty() {
			self.buffer[victim].insert(&mut bin);
		} else {
			self.stash.push(bin);
		}

		self.len += 1;
		true
	}

	/// 判断一个元素是否在 Table 中
	pub fn contains(&self, data: u32) -> bool {
		let bin = Bin::from_slice(&data.to_le_bytes());
		let hset = get_two_hash(bin.as_ref(), self.capacity);

		self.buffer[hset.0].contains(&bin) ||
		self.buffer[hset.1].contains(&bin) ||
		self.stash.iter().any(|item| item == &bin)
	}

	pub fn stash_size(&self) -> usize {
		self.stash.len()
	}

	fn try_put(&mut self, bin: &mut Bin, idx: usize) -> bool {
		if self.buffer[idx].is_empty() {
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
		let rate = 2.4f64;
		let f = (1 << 20) as f64 * rate;
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

		println!("swell rate: {}", rate);
		println!("table capacity: {}", ctable.capacity);
		println!("element count: {}", ctable.len);
		println!("use stash: {}", ctable.stash_size());
		println!("record: {:?}", &ctable.record);
	}
}