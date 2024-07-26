use super::mem_manager_type::MemoryManagerType;


trait JitMemoryManager {
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