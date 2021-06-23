use crate::mem_manage;
use alloc::vec::Vec;
use core::{convert::TryFrom, mem, ptr};

/// Round `size` up to the nearest multiple of `page_size`.
fn round_up_to_page_size(size: usize, page_size: usize) -> usize {
    (size + (page_size - 1)) & !(page_size - 1)
}

/// A simple struct consisting of a pointer and length.
struct PtrLen {
    ptr: *mut u8,
    len: usize,
}

impl PtrLen {
    /// Create a new empty `PtrLen`.
    fn new() -> Self {
        Self {
            ptr: ptr::null_mut(),
            len: 0,
        }
    }

    fn with_size(size: usize) -> Result<Self, ()> {
        let page_size = mem_manage().page_size();
        let alloc_size = round_up_to_page_size(size, page_size);
        unsafe {
            let ptr = mem_manage().alloc_page_aligned(alloc_size);
            Ok(Self {
                ptr,
                len: alloc_size,
            })
        }
    }
}

impl Drop for PtrLen {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                mem_manage().set_rw(self.ptr, self.len);
                mem_manage().dealloc(self.ptr, self.len);
            }
        }
    }
}

/// JIT memory manager. This manages pages of suitably aligned and
/// accessible memory. Memory will be leaked by default to have
/// function pointers remain valid for the remainder of the
/// program's life.
pub(crate) struct Memory {
    allocations: Vec<PtrLen>,
    executable: usize,
    current: PtrLen,
    position: usize,
}

impl Memory {
    pub(crate) fn new() -> Self {
        Self {
            allocations: Vec::new(),
            executable: 0,
            current: PtrLen::new(),
            position: 0,
        }
    }

    fn finish_current(&mut self) {
        self.allocations
            .push(mem::replace(&mut self.current, PtrLen::new()));
        self.position = 0;
    }

    pub(crate) fn allocate(&mut self, size: usize, align: u64) -> Result<*mut u8, ()> {
        let align = usize::try_from(align).expect("alignment too big");
        if self.position % align != 0 {
            self.position += align - self.position % align;
            debug_assert!(self.position % align == 0);
        }

        if size <= self.current.len - self.position {
            // TODO: Ensure overflow is not possible.
            let ptr = unsafe { self.current.ptr.add(self.position) };
            self.position += size;
            return Ok(ptr);
        }

        self.finish_current();

        // TODO: Allocate more at a time.
        self.current = PtrLen::with_size(size)?;
        self.position = size;
        Ok(self.current.ptr)
    }

    /// Set all memory allocated in this `Memory` up to now as readable and executable.
    pub(crate) fn set_readable_and_executable(&mut self) {
        self.finish_current();

        {
            for &PtrLen { ptr, len } in &self.allocations[self.executable..] {
                if len != 0 {
                    mem_manage().set_rx(ptr, len);
                }
            }
        }
    }

    /// Set all memory allocated in this `Memory` up to now as readonly.
    pub(crate) fn set_readonly(&mut self) {
        self.finish_current();

        {
            for &PtrLen { ptr, len } in &self.allocations[self.executable..] {
                if len != 0 {
                    mem_manage().set_r(ptr, len);
                }
            }
        }
    }

    /// Frees all allocated memory regions that would be leaked otherwise.
    /// Likely to invalidate existing function pointers, causing unsafety.
    pub(crate) unsafe fn free_memory(&mut self) {
        self.allocations.clear();
    }
}

impl Drop for Memory {
    fn drop(&mut self) {
        // leak memory to guarantee validity of function pointers
        mem::replace(&mut self.allocations, Vec::new())
            .into_iter()
            .for_each(mem::forget);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_up_to_page_size() {
        assert_eq!(round_up_to_page_size(0, 4096), 0);
        assert_eq!(round_up_to_page_size(1, 4096), 4096);
        assert_eq!(round_up_to_page_size(4096, 4096), 4096);
        assert_eq!(round_up_to_page_size(4097, 4096), 8192);
    }
}
