use super::jit_mem_block::JitMemoryBlock;

pub(crate) trait JitMemoryAllocator {
    fn allocate(&self, size: u64) -> Box<dyn JitMemoryBlock>;
    fn reserve(&self, size: u64) -> Box<dyn JitMemoryBlock>;
}