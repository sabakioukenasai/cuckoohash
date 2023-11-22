#[macro_export]
macro_rules! define_bucket_bin {
	($s: expr) => {
		const BIN_SIZE: usize = ($s + 7) >> 3;
		const BUCKET_SIZE: usize = 1;

		#[derive(Clone, Debug, PartialEq)]
		pub struct Bin {
			data: [u8; BIN_SIZE]
		}

		impl Bin {
			/// 返回一个空的Bin
			pub fn new() -> Self {
				Self {
					data: [0; BIN_SIZE]
				}
			}

			/// 使用字节切片构建一个Bin，如果切片长度超过了一个Bin的长度
			/// 则只会选取前 BIN_SIZE 个字节；如果不足一个Bin的长度
			/// 则只填充Bin的前一部分
			pub fn from_slice(slice: &[u8]) -> Self {
				let mut bin = Self::new();
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

		#[derive(Clone, Debug)]
		pub struct Bucket {
			buffer: [Option<Bin>; BUCKET_SIZE]
		}

		impl Bucket {
			/// 返回一个空Bucket
			pub fn new() -> Self {
				Self {
					buffer: [None; BUCKET_SIZE]
				}
			}

			/// 询问Bucket中是否有空位
			pub fn is_available(&self) -> bool {
				self.buffer.iter().any(|bin| bin.is_none())
			}

			/// 判断一个 Bin 是否在 Bucket 中
			pub fn contains(&self, bin: &Bin) -> bool {
				self.buffer.iter().filter(|item| item.is_some())
					.map(|item| item.as_ref().unwrap())
					.any(|item| item == bin)
			}

			/// 向Bucket中插入一个Bin，并且返回原来的Bin
			pub fn insert(&mut self, bin: Bin) -> Option<Bin> {
				for i in 0..BUCKET_SIZE {
					if self.buffer[i].is_none() {
						self.buffer[i] = Some(bin);
						return None;
					}
				}
				let victim = get_random() % BUCKET_SIZE;
				let old = self.buffer[victim].take();
				self.buffer[victim] = Some(bin);
				old
			}

			/// 
			pub fn delete(&mut self, _key: Bin) -> bool {
				unimplemented!()
			}
		}
	};
}