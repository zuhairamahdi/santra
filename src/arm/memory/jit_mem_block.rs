pub trait JitMemoryBlock: std::ops::Drop {
    fn pointer(&self) -> *mut std::ffi::c_void;

    fn commit(&self, offset: u64, size: u64);

    fn map_as_rw(&self, offset: u64, size: u64);
    fn map_as_rx(&self, offset: u64, size: u64);
    fn map_as_rwx(&self, offset: u64, size: u64);
}