pub fn malloc(len: usize, align: usize) -> isize {
    syscall!(10001, len, align)
}

pub fn free(addr: usize, len: usize, align: usize) -> isize {
    syscall!(10005, addr, len, align)
}

pub fn physmap(vaddr: usize, paddr: usize, len: usize) -> isize {
    syscall!(10002, vaddr, paddr, len)
}

use core::alloc::Layout;

#[global_allocator]
static ALLOCATOR: MemoryAllocator = MemoryAllocator;

struct MemoryAllocator;

unsafe impl alloc::alloc::GlobalAlloc for MemoryAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        malloc(layout.size(), layout.align()) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        free(ptr as usize, layout.size(), layout.align());
    }
}

/// A safe virtual mapping to physical memory that unmaps the memory when the structure goes out
/// of scope.
///
/// This function provides a safe binding to [physmap]. It implements Drop to free the mapped memory
/// when the structure goes out of scope.
pub struct PhysBorrowed {
    mem: *mut (),
    len: usize,
}
impl PhysBorrowed {
    /// Constructs a PhysBorrowed instance.
    ///
    /// # Arguments
    /// See [physmap] for a description of the parameters.
    ///
    /// # Returns
    /// A '[Result]' which contains the following:
    /// - A '[PhysBorrowed]' which represents the newly mapped region.
    /// - An 'Err' if a memory mapping error occurs.
    ///
    /// # Errors
    /// See [physmap] for a description of the error cases.
    pub fn map(base_phys: usize, len: usize) -> Result<Self, ()> {
        physmap(base_phys, base_phys, len);
        Ok(Self {
            mem: base_phys as *mut (),
            len: len.next_multiple_of(4096),
        })
    }

    /// Gets a raw pointer to the borrowed region.
    ///
    /// # Returns
    /// - self.mem - A pointer to the mapped region in virtual memory.
    ///
    /// # Notes
    /// - The pointer may live beyond the lifetime of [PhysBorrowed], so dereferences to the pointer
    ///   must be treated as unsafe.
    ///
    pub fn as_ptr(&self) -> *mut () {
        self.mem
    }

    /// Gets the length of the mapped region.
    ///
    /// # Returns
    /// - self.len - The length of the mapped region. It should be a multiple of [PAGE_SIZE]
    pub fn mapped_len(&self) -> usize {
        self.len
    }
}

// impl Drop for PhysBorrowed {
//     /// Frees the mapped memory region.
//     fn drop(&mut self) {
//         unsafe {
//             let _ = libreduct::call::munmap(self.mem, self.len);
//         }
//     }
// }
