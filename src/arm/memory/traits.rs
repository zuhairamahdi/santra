use super::mem_manager_type::MemoryManagerType;


pub trait JitMemoryBlock: std::ops::Drop {
    fn pointer(&self) -> *mut std::ffi::c_void;

    fn commit(&self, offset: u64, size: u64);

    fn map_as_rw(&self, offset: u64, size: u64);
    fn map_as_rx(&self, offset: u64, size: u64);
    fn map_as_rwx(&self, offset: u64, size: u64);
}

pub(crate) trait JitMemoryAllocator {
    fn allocate(&self, size: u64) -> Box<dyn JitMemoryBlock>;
    fn reserve(&self, size: u64) -> Box<dyn JitMemoryBlock>;
}

pub trait JitMemoryManager {
    fn address_space_bits(&self) -> i32;
    fn page_table_pointer(&self) -> *mut std::ffi::c_void;
    fn memory_manager_type(&self) -> MemoryManagerType;
    fn unmap_event(&self) -> Box<dyn FnMut(u64, u64)>;
    fn read<T>(&self, va: u64) -> T where T: Copy;
    fn read_tracked<T>(&self, va: u64) -> T where T: Copy;
    fn read_guest<T>(&self, va: u64) -> T where T: Copy;
    fn write<T>(&self, va: u64, value: T) where T: Copy;
    fn write_guest<T>(&self, va: u64, value: T) where T: Copy;
    fn get_span(&self, va: u64, size: i32, tracked: bool) -> &[u8];
    fn get_ref<T>(&self, va: u64) -> &T where T: Copy;
    fn is_mapped(&self, va: u64) -> bool;
    fn signal_memory_tracking(&self, va: u64, size: u64, write: bool, precise: bool, exempt_id: Option<i32>);
}
