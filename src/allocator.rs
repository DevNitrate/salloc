use std::{ffi::c_void, marker::PhantomData, ptr};
use windows_sys::Win32::{Foundation::BOOL, System::Memory::*};

pub struct Sallocator<T> {
    size: usize,
    free: *mut c_void,
    addr: *mut T,
    phantom: PhantomData<T>
}

impl<T: std::fmt::Debug + Clone + Copy> Sallocator<T> {
    pub fn new(size: usize) -> Self {
        let addr: *mut T = ptr::null_mut();
        let free: *mut c_void = ptr::null_mut();

        Sallocator {
            size,
            free,
            addr,
            phantom: PhantomData
        }
    }

    pub fn salloc(&mut self) {
        unsafe {
            self.free = VirtualAlloc(self.free, self.size, MEM_COMMIT, PAGE_READWRITE);
            self.addr = self.free.cast();
        }
    }

    pub fn set(&mut self, index: isize, el: T) {
        unsafe {
            *self.addr.offset(index) = el;
        }
    }

    pub fn get(&mut self, index: isize) -> T {
        unsafe {
            *self.addr.offset(index)
        }
    }

    pub fn free(&mut self) {
        if self.free.is_null() {
            panic!("cannot free memory that has already been freed");
        }

        unsafe {
            let free: BOOL = VirtualFree(self.free, self.size, MEM_RELEASE);

            if free.is_negative() {
                panic!("failed to free memory");
            }
        }

        self.free = ptr::null_mut();
    }
}