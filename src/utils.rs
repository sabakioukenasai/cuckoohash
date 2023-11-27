use std::mem::size_of;
use ring::{digest, rand};

/// get 2 hash results
pub fn get_two_hash(key: &[u8], range: usize) -> (usize, usize)
{
	assert!(range != 0);

	let hash = digest::digest(&digest::SHA256, key);
	let hash = hash.as_ref().as_ptr() as *const usize;
	
	let (mut h0, mut h1) = (0, 0);
	unsafe {
		std::ptr::write(&mut h0 as *mut usize, *hash);
		std::ptr::write(&mut h1 as *mut usize, *hash.wrapping_add(1));
	}
	(h0 % range, h1 % range)
}

/// get 3 hash results
pub fn get_three_hash(key: &[u8], range: usize) -> (usize, usize, usize)
{
	assert!(range != 0);

	let hash = digest::digest(&digest::SHA256, key);
	let hash = hash.as_ref().as_ptr() as *const usize;
	
	let (mut h0, mut h1, mut h2) = (0, 0, 0);
	unsafe {
		std::ptr::write(&mut h0 as *mut usize, *hash);
		std::ptr::write(&mut h1 as *mut usize, *hash.wrapping_add(1));
		std::ptr::write(&mut h2 as *mut usize, *hash.wrapping_add(2));
	}
	(h0 % range, h1 % range, h2 % range)
}

/// return an alternate hash not equal to the given one
pub fn get_alt_hash(data: &[u8], range: usize, hash: usize) -> usize
{
	let hset = get_two_hash(data, range);
	if hset.0 == hash {
		hset.1
	} else {
		hset.0
	}
}

/// return an alternate hash not equal to the given one
pub fn get_alt_hash_three(data: &[u8], range: usize, hash: usize) -> usize
{
	let hset = get_three_hash(data, range);
	if hset.0 == hash {
		hset.1
	} else if hset.1 == hash {
		hset.2
	} else {
		hset.0
	}
}

/// 返回 [0, `range` - 1] 范围内的一个随机数
pub fn get_random(range: usize) -> usize {
	assert!(range != 0);
	let rng = rand::SystemRandom::new();
	usize::from_le_bytes(
		rand::generate::<[u8; size_of::<usize>()]>(&rng).unwrap().expose()
	) % range
}