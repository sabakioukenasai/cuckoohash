use std::mem::size_of;
use ring::{digest, rand};

/// 输入字节串，将其映射成两个在 [0, `range`-1] 范围内呃正整数
pub fn get_double_hash(key: &[u8], range: usize) -> (usize, usize)
{
	let hash = digest::digest(&digest::SHA256, key);
	let hash = hash.as_ref().as_ptr() as *const usize;

	let (mut h1, mut h2) = (0, 0);
	unsafe {
		std::ptr::write(&mut h1 as &mut usize, *hash);
		std::ptr::write(&mut h2 as &mut usize, *hash.wrapping_add(1));
	}
	if range != 0 {
		(h1 % range, h2 % range)
	} else {
		(h1, h2)
	}
}

/// 返回另一个 Hash 值 
pub fn get_another_hash(data: &[u8], range: usize, hash: usize) -> usize 
{
	let (h1, h2) = get_double_hash(data, range);
	if h1 as usize == hash {
		h2 as usize
	} else {
		h1 as usize
	}
}

/// 返回 [0, `range` - 1] 范围内的一个随机数
pub fn get_random(range: usize) -> usize {
	let rng = rand::SystemRandom::new();
	let res = usize::from_le_bytes(
		rand::generate::<[u8; size_of::<usize>()]>(&rng).unwrap().expose()
	);
	if range != 0 {
		res % range
	} else {
		res
	}
}

#[cfg(test)]
mod test {

use super::*;

	#[test]
	fn test_get_random() {
		println!("{:?}", get_random(87).to_le_bytes());
	}

	#[test]
	fn test_get_hash() {
		let start = 0000u32;
		let len = 65536u32;
		let m = (len as f64 * 2.4) as usize;
		let step = m / 100;
		let mut count = vec![0; 101];

		for i in start..start + len {
			let (h1, h2) = get_double_hash(&i.to_le_bytes(), m);
			count[(h1 / step) as usize] += 1;
			count[(h2 / step) as usize] += 1;
			// println!("{:?}, {:?}", h1 % m, h2 % m);
			// println!("{:?}, {:?}", (h1 % _MOD).to_be_bytes(), (h2 % _MOD).to_be_bytes());
		}
		println!("{:?}", &count);
	}

	#[test]
	fn test_get_another_hash() {
		let number = 289374u32;
		let (h1, h2) = get_double_hash(&number.to_be_bytes(), 320872);

		assert_eq!(h1, get_another_hash(&number.to_be_bytes(), 320872, h2));
		assert_eq!(h2, get_another_hash(&number.to_be_bytes(), 320872, h1));
	}
}