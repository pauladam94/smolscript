use std::{
    alloc::{GlobalAlloc, Layout, System},
    sync::atomic::{AtomicUsize, Ordering::Relaxed},
};

#[derive(Default)]
pub struct CounterAlloc {
    pub allocated: AtomicUsize,
    pub max_allocated: AtomicUsize,
}

impl CounterAlloc {
    pub const fn new() -> Self {
        CounterAlloc {
            allocated: AtomicUsize::new(0),
            max_allocated: AtomicUsize::new(0),
        }
    }
}

unsafe impl GlobalAlloc for CounterAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret = unsafe { System.alloc(layout) };
        if !ret.is_null() {
            self.allocated.fetch_add(layout.size(), Relaxed);
            self.max_allocated
                .fetch_max(self.allocated.load(Relaxed), Relaxed);
        }
        ret
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe {
            System.dealloc(ptr, layout);
        }
        self.allocated.fetch_sub(layout.size(), Relaxed);
    }
}

/// Number of Bytes pretty printer.
///
/// - 234 bytes        is written 234 bytes
/// - 23 000 bytes     is written     23 Ko
/// - 42 000 000 bytes is written     42 Mo
pub struct PPBytes(pub usize);

impl std::fmt::Display for PPBytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self(n) = self;
        if *n < 1000 {
            write!(f, "{} bytes", self)
        } else if *n < 1000000 {
            write!(f, "{} Ko {} bytes", n / 1000, n % 1000)
        } else if *n < 1000000000 {
            write!(f, "{} Mo {} Ko", n / 1000000, n % 1000000)
        } else {
            write!(f, "{} Go {} Mo", n / 1000000000, n % 1000000000)
        }
    }
}
