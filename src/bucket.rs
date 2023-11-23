use crate::utils as ut;

const ELEM_BIT_WIDTH: usize = 32;
const BIN_SIZE: usize = (ELEM_BIT_WIDTH + 7) >> 3;
const BUCKET_SIZE: usize = 1;

const DUMMY_BIN: Bin = Bin {
	data: [0xa5; BIN_SIZE]
};

/// 储存元素的最小单元，每个 Bin 内储存一个元素。
#[derive(Clone, Debug, PartialEq)]
pub struct Bin {
	data: [u8; BIN_SIZE]
}

impl Bin {
	/// 返回一个空的Bin
	pub fn empty() -> Self {
		DUMMY_BIN
	}

	/// if a Bin is filled with dummy data, it's a empty Bin
	pub fn is_empty(&self) -> bool {
		self == &DUMMY_BIN
	}

	/// 使用字节切片构建一个Bin，如果切片长度超过了一个Bin的长度
	/// 则只会选取前 BIN_SIZE 个字节；如果不足一个Bin的长度
	/// 则只填充Bin的前一部分
	pub fn from_slice(slice: &[u8]) -> Self {
		let mut bin = Self::empty();
		bin.slice_copy(slice);
		bin
	}

	/// 使用字节切片构建一个Bin，如果切片长度超过了一个Bin的长度
	/// 则只会选取前 BIN_SIZE 个字节；如果不足一个Bin的长度
	/// 则只填充Bin的前一部分
	pub fn slice_copy(&mut self, slice: &[u8]) {
		if slice.len() < BIN_SIZE {
			self.data[..slice.len()].copy_from_slice(&slice[..]);
		} else {
			self.data.copy_from_slice(&slice[..BIN_SIZE]);
		}
	}
}

impl AsRef<[u8]> for Bin {
	fn as_ref(&self) -> &[u8] {
		&self.data
	}
}

#[derive(Clone, Debug)]
pub struct Bucket {
	buffer: [Bin; BUCKET_SIZE]
}

impl Bucket {
	/// 返回一个空Bucket
	pub fn new() -> Self {
		Self {
			buffer: [Bin::empty(); BUCKET_SIZE]
		}
	}

	/// Check if there are empty `Bin` in the `Bucket`
	pub fn is_available(&self) -> bool {
		self.buffer.iter().any(|bin| bin.is_empty())
	}

	/// Check if 'data' is in the Bucket
	pub fn contains(&self, bin: &Bin) -> bool {
		self.buffer.iter()
			.any(|item| item == bin)
	}

	/// Insert a `Bin` into `Bucket`, consuming the original `data`
	/// and assign it with the `data` in the Bin before.
	pub fn insert(&mut self, bin: &mut Bin) {
		if BUCKET_SIZE > 1 {
			for i in 0..BUCKET_SIZE {
				if self.buffer[i].is_empty() {
					std::mem::swap(&mut self.buffer[i], bin);
					return ;
				}
			}
			let victim = ut::get_random(BUCKET_SIZE);
			std::mem::swap(&mut self.buffer[victim], bin);
		} else {
			std::mem::swap(&mut self.buffer[0], bin);
		}
	}
}
