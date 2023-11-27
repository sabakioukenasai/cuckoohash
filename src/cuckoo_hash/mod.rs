pub mod standard;
pub mod threeway;

/// 最大重分配次数
pub const MAX_RELOCATE: usize = 256;

/// 默认哈希表大小
pub const DEFAULT_CAPACITY: usize = 1 << 12;

/// 默认元素大小
pub const DEFAULT_ELEM_BYTE_WIDTH: usize = 32;

/// 默认 stash 大小
pub const DEFAULT_STASH_SIZE: usize = 2;

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