use x86_64::{PhysAddr, VirtAddr};

pub struct DmaManager;

impl DmaManager {
    pub const UNIT_SIZE: usize = 4096;

    pub fn allocate(size: usize) -> (PhysAddr, VirtAddr) {
        let phys = syscall!(10008, size) as usize;
        crate::mm::physmap(phys, phys, size);
        (PhysAddr::new(phys as u64), VirtAddr::new(phys as u64))
    }

    pub fn deallocate(addr: VirtAddr) {
        syscall!(10009, addr.as_u64() as usize);
    }
}
