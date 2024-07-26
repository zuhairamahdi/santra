pub enum MemoryManagerType {
    SoftwareMmu,
    SoftwarePageTable,
    HostMapped,
    HostMappedUnsafe,
    HostTracked,
    HostTrackedUnsafe,
}

impl MemoryManagerType {
    pub fn is_host_mapped(&self) -> bool {
        match self {
            MemoryManagerType::HostMapped | MemoryManagerType::HostMappedUnsafe => true,
            _ => false,
        }
    }

    pub fn is_host_tracked(&self) -> bool {
        match self {
            MemoryManagerType::HostTracked | MemoryManagerType::HostTrackedUnsafe => true,
            _ => false,
        }
    }

    pub fn is_host_mapped_or_tracked(&self) -> bool {
        self.is_host_mapped() || self.is_host_tracked()
    }
}