//! Simple static bump allocator for bare-metal #![no_std] environment.

use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicUsize, Ordering};

const HEAP_SIZE: usize = 64 * 1024; // 64 KB heap
static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

pub struct BumpAllocator {
    next: AtomicUsize,
}

impl BumpAllocator {
    pub const fn new() -> Self {
        Self {
            next: AtomicUsize::new(0),
        }
    }
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let align = layout.align();
        let size = layout.size();

        loop {
            let current = self.next.load(Ordering::Relaxed);
            let heap_start = core::ptr::addr_of!(HEAP) as usize;
            let current_addr = heap_start + current;
            let align_offset = (align - (current_addr % align)) % align;
            let new_offset = current + align_offset + size;

            if new_offset > HEAP_SIZE {
                return core::ptr::null_mut();
            }

            if self
                .next
                .compare_exchange_weak(
                    current,
                    new_offset,
                    Ordering::SeqCst,
                    Ordering::Relaxed,
                )
                .is_ok()
            {
                return (heap_start + current + align_offset) as *mut u8;
            }
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Bump allocator does not reclaim individual allocations
    }
}

#[global_allocator]
static ALLOCATOR: BumpAllocator = BumpAllocator::new();
