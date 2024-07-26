use super::{jit_mem_allocator::JitMemoryAllocator, jit_mem_block::JitMemoryBlock};

pub struct ReservedRegion {
    block: Box<dyn JitMemoryBlock>,
    max_size: u64,
    size_granularity: u64,
    current_size: u64,
    lock: std::sync::Mutex<()>,
}

impl ReservedRegion {
    pub const DefaultGranularity: u64 = 65536; // Mapping granularity in Windows.

    pub fn new(allocator: &dyn JitMemoryAllocator, max_size: u64, granularity: u64) -> Self {
        let granularity = if granularity == 0 {
            Self::DefaultGranularity
        } else {
            granularity
        };

        let block = allocator.reserve(max_size);
        Self {
            block,
            max_size,
            size_granularity: granularity,
            current_size: 0,
            lock: std::sync::Mutex::new(()),
        }
    }

    pub fn expand_if_needed(&mut self, desired_size: u64) {
        if desired_size > self.max_size {
            panic!("Out of memory");
        }

        if desired_size > self.current_size {
            // Lock, and then check again. We only want to commit once.
            let _lock = self.lock.lock().unwrap();
            if desired_size >= self.current_size {
                let overflow_bytes = desired_size - self.current_size;
                let more_to_commit = (((self.size_granularity - 1) + overflow_bytes) / self.size_granularity) * self.size_granularity; // Round up.
                self.block.commit(self.current_size, more_to_commit);
                self.current_size += more_to_commit;
            }
        }
    }
}