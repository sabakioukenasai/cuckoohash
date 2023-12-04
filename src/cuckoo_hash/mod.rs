pub mod standard;
pub mod threeway;

/// 最大重分配次数
const MAX_RELOCATE: usize = 256;

/// 默认哈希表大小
const DEFAULT_CAPACITY: usize = 1 << 12;

/// 默认 stash 大小
const DEFAULT_STASH_SIZE: usize = 2;

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