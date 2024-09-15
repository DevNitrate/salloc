use std::{ffi::c_void, marker::PhantomData, ptr};
use windows_sys::Win32::{Foundation::{BOOL, HANDLE}, System::Memory::*};

pub struct Sallocator<T> {
    size: usize,
    free: *mut c_void,
    addr: *mut T,
    heap: HANDLE,
    phantom: PhantomData<T>
}

#[derive(Debug)]
pub enum HeapError {
    SallocCreateFailed,
    SallocMallocFailed,
    SallocReallocFailed,
    SallocSetUnallocated,
    SallocGetUnallocated,
    SallocFreeUnallocated,
    SallocSetOutOfBound,
    SallocGetOutOfBound,
    SallocFreeFailed,
    SallocDestroyFailed
}

impl std::fmt::Display for HeapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::SallocCreateFailed => write!(f, "SallocCreateFailed"),
            Self::SallocMallocFailed => write!(f, "SallocMallocFailed"),
            Self::SallocReallocFailed => write!(f, "SallocReallocFailed"),
            Self::SallocSetUnallocated => write!(f, "SallocSetUnallocated"),
            Self::SallocGetUnallocated => write!(f, "SallocGetUnallocated"),
            Self::SallocFreeUnallocated => write!(f, "SallocFreeUnallocated"),
            Self::SallocSetOutOfBound => write!(f, "SallocSetOutOfBound"),
            Self::SallocGetOutOfBound => write!(f, "SallocGetOutOfBound"),
            Self::SallocFreeFailed => write!(f, "SallocFreeFailed"),
            Self::SallocDestroyFailed => write!(f, "SallocDestroyFailed"),
        }
    }
}

impl<T: Clone + Copy> Sallocator<T> {
    pub fn new() -> Result<Self, HeapError> {
        let addr: *mut T = ptr::null_mut();
        let free: *mut c_void = ptr::null_mut();
        let heap: HANDLE;
        unsafe {
            heap = HeapCreate(HEAP_CREATE_ENABLE_EXECUTE, 0, 0);
        }

        if heap.is_null() {
            return Err(HeapError::SallocCreateFailed);
        }

        return Ok(Sallocator {
            size: 0,
            free,
            addr,
            heap,
            phantom: PhantomData
        });
    }

    pub fn malloc(&mut self, size: usize) -> Result<(), HeapError> {
        self.size = size;

        unsafe {
            self.free = HeapAlloc(self.heap, HEAP_ZERO_MEMORY, self.size);
            self.addr = self.free.cast();
        }

        if self.free.is_null() {
            return Err(HeapError::SallocMallocFailed);
        } else {
            return Ok(());
        }
    }

    pub fn realloc(&mut self, size: usize) -> Result<(), HeapError> {
        self.size = size;

        unsafe {
            self.free = HeapReAlloc(self.heap, HEAP_ZERO_MEMORY, self.free, self.size);
            self.addr = self.free.cast();
        }

        if self.free.is_null() {
            return Err(HeapError::SallocReallocFailed);
        } else {
            return Ok(());
        }
    }

    pub fn set(&mut self, index: isize, el: T) -> Result<(), HeapError> {
        if self.addr.is_null() {
            return Err(HeapError::SallocSetUnallocated);
        }

        if index as usize >= self.size / size_of::<T>() as usize || index < 0 {
            return Err(HeapError::SallocSetOutOfBound);
        }

        unsafe {
            *self.addr.offset(index) = el;
        }

        return Ok(());
    }

    pub fn get(&mut self, index: isize) -> Result<T, HeapError> {
        if self.addr.is_null() {
            return Err(HeapError::SallocGetUnallocated);
        }

        if index as usize >= self.size / size_of::<T>() as usize || index < 0 {
            return Err(HeapError::SallocGetOutOfBound);
        }

        unsafe {
            return Ok(*self.addr.offset(index));
        }
    }

    pub fn free(&mut self) -> Result<(), HeapError> {
        if self.free.is_null() {
            return Err(HeapError::SallocFreeUnallocated);
        }

        unsafe {
            let free: BOOL = HeapFree(self.heap, 0, self.free);
    
            self.free = ptr::null_mut();
            self.addr = ptr::null_mut();

            if free == 0 {
                return Err(HeapError::SallocFreeFailed);
            } else {
                return Ok(());
            }
        }
    }

    pub fn destroy(&mut self) -> Result<(), HeapError> {
        unsafe {
            let free: BOOL = HeapDestroy(self.heap);
    
            self.free = ptr::null_mut();
            self.addr = ptr::null_mut();
            self.size = 0;

            if free == 0 {
                return Err(HeapError::SallocDestroyFailed);
            } else {
                return Ok(());
            }
        }
    }
}