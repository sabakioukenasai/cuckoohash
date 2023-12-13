use crate::utils::*;
use crate::bucket::*;
use super::*;

use std::iter::repeat;

/// A data storage hash table with immutable size, using cuckoo strategy
/// to handle collisions.
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
	/// Constructs a new, empty two-way cuckoohash table with default capacity 
	/// and stash size.
	/// 
	/// Renamed as `StandardCuckoo` as each of the elements inserted into the
	/// table has two alternative locations.
	/// 
	/// # Example
	/// ```rust
	/// use cuckoohash::StandardCuckoo;
	/// 
	/// let ctb = StandardCuckoo::new();
	/// assert_eq!(ctb.len(), 0);
	/// ```
	pub fn new() -> Self {
		Self {
			buffer: repeat(Bucket::new())
				.take(DEFAULT_CAPACITY)
				.collect::<Vec<_>>()
				.into_boxed_slice(),
			capacity: DEFAULT_CAPACITY,
			stash: Vec::<Bin>::with_capacity(DEFAULT_STASH_SIZE),
			len: 0,
			#[cfg(test)]
			record: Recorder::new()
		}
	}

	/// Constructs a new, empty two-way cuckoohash table with at least
	/// the specified capacity.
	/// 
	/// # Examples
	/// ```rust
	/// use cuckoohash::StandardCuckoo;
	/// 
	/// let ctb = StandardCuckoo::with_capacity(1024);
	/// assert!(ctb.capacity() >= 1024);
	/// ```
	pub fn with_capacity(cap: usize) -> Self {
		let capacity = std::cmp::max(1, cap);
		let buffer = repeat(Bucket::new())
				.take(capacity)
				.collect::<Vec<_>>()
				.into_boxed_slice();
		let stash = Vec::with_capacity(DEFAULT_STASH_SIZE);

		Self {
			capacity: buffer.len(),
			buffer,
			stash,
			len: 0,
			#[cfg(test)]
			record: Recorder::new()
		}
	}

	/// Inserts an element into the cuckoohash table. 
	/// 
	/// Returns `ture` if successfully insert the element into
	/// buckets, or `false` if collisions occurred and the element
	/// is stored in the stash.
	/// 
	/// # Warning
	/// Inserting duplicate elements into the cuckoo table will make
	/// no change to the table and return `true`.
	/// 
	/// # Examples
	/// ```rust
	/// use cuckoohash::StandardCuckoo;
	/// 
	/// let mut ct = StandardCuckoo::new();
	/// assert!(ct.insert(3));
	/// assert_eq!(ct.len(), 1);
	/// assert!(ct.insert(4));
	/// assert_eq!(ct.len(), 2);
	/// assert!(ct.insert(3));
	/// assert_eq!(ct.len(), 2);
	/// 
	/// let mut smallct = StandardCuckoo::with_capacity(1);
	/// 
	/// assert!(smallct.insert(2));
	/// assert_eq!(smallct.len(), 1);
	/// assert!(!smallct.insert(4));
	/// assert_eq!(smallct.len(), 2);
	/// ```
	pub fn insert(&mut self, data: u32) -> bool {
		let mut bin = Bin::from_slice(&data.to_le_bytes());
		let hset = get_two_hash(bin.as_ref(), self.capacity);
		
		if self.try_put(&mut bin, hset.0) 
		|| self.try_put(&mut bin, hset.1) {
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
			self.buffer[victim].insert(&mut bin, 0);
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

		match self.buffer[victim].get_room() {
			Some(i) => self.buffer[victim].insert(&mut bin, i),
			None => { self.stash.push(bin); self.len += 1; return false}
		}

		self.len += 1;
		true
	}

	/// Returns `true` if an element in the table.
	/// 
	/// There is a negligible possibility that an element not in the
	/// table will be recognized as being present in the table.
	pub fn contains(&self, data: u32) -> bool {
		let bin = Bin::from_slice(&data.to_le_bytes());
		let hset = get_two_hash(bin.as_ref(), self.capacity);

		self.buffer[hset.0].contains(&bin) ||
		self.buffer[hset.1].contains(&bin) ||
		self.stash.iter().any(|item| item == &bin)
	}

	/// Returns the size of the stash.
	pub fn stash_len(&self) -> usize {
		self.stash.len()
	}

	/// Returns the capacity of the cuckoohash table.
	pub fn capacity(&self) -> usize {
		self.capacity
	}

	/// Returns the number of the elements in the table.
	pub fn len(&self) -> usize {
		self.len
	}

	fn try_put(&mut self, bin: &mut Bin, idx: usize) -> bool {
		match self.buffer[idx].get_room() {
			Some(i) => {
				self.buffer[idx].insert(bin, i);
				self.len += 1;
				true
			}
			None => self.buffer[idx].contains(bin),
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

		println!("expension rate: {}", rate);
		println!("table capacity: {}", ctable.capacity);
		println!("element count: {}", ctable.len);
		println!("use stash: {}", ctable.stash_len());
		println!("record: {:?}", &ctable.record);
	}
}