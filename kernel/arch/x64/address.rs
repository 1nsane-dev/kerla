use core::fmt;

/// The base virtual address of straight mapping.
const KERNEL_BASE_ADDR: u64 = 0xffff_8000_0000_0000;

/// The end of straight mapping. Any physical address `P` is mapped into the
/// kernel's virtual memory address `KERNEL_BASE_ADDR + P`.
const KERNEL_STRAIGHT_MAP_PADDR_END: u64 = 0x1_0000_0000;

/// Represents a physical memory address.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct PAddr(u64);

impl PAddr {
    pub const fn new(addr: u64) -> PAddr {
        PAddr(addr)
    }

    pub const unsafe fn as_ptr<T>(self) -> *const T {
        assert!(self.0 < KERNEL_STRAIGHT_MAP_PADDR_END);
        (self.0 + KERNEL_BASE_ADDR) as *const _
    }

    pub const unsafe fn as_mut_ptr<T>(self) -> *mut T {
        assert!(self.0 < KERNEL_STRAIGHT_MAP_PADDR_END);
        (self.0 + KERNEL_BASE_ADDR) as *mut _
    }

    #[inline(always)]
    #[must_use]
    pub const fn add(self, offset: usize) -> PAddr {
        PAddr(self.0 + offset as u64)
    }
}

/// Represents a *kernel* virtual memory address.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct VAddr(u64);

impl VAddr {
    pub const fn new(addr: u64) -> VAddr {
        VAddr(addr)
    }

    pub const unsafe fn as_ptr<T>(self) -> *const T {
        assert!(self.0 >= KERNEL_BASE_ADDR);
        self.0 as *const _
    }

    pub const unsafe fn as_mut_ptr<T>(self) -> *mut T {
        assert!(self.0 >= KERNEL_BASE_ADDR);
        self.0 as *mut _
    }

    #[inline(always)]
    pub const fn value(self) -> usize {
        self.0 as usize
    }

    #[inline(always)]
    #[must_use]
    pub const fn add(self, offset: usize) -> VAddr {
        VAddr(self.0 + offset as u64)
    }
}

impl fmt::Display for VAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:016x}", self.value())
    }
}
