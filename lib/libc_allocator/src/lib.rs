// A custom allocator for with the libc interface (using malloc and friends). This
// can be used in place of alloc_system on systems that don't have it provided. like
// bare metal ARMs for example.

// The compiler needs to be instructed that this crate is an allocator in order
// to realize that when this is linked in another allocator like jemalloc should
// not be linked in
#![feature(allocator)]
#![allocator]

// Allocators are not allowed to depend on the standard library which in turn
// requires an allocator in order to avoid circular dependencies. This crate,
// however, can use all of libcore.
#![no_std]

// Use this critical section package to make memory allocations safe from interrupts.
// This is not safe for multi-threading (as a fully re-entrant version of malloc would be),
//but it works for a single thread with interrupts.
extern crate critical_section_arm;
use critical_section_arm::CriticalSection;

//Define some types for our 32-bit processor.
type usize = u32;
type c_void = u8;

extern {
    pub fn malloc(size: u32) -> *mut c_void;
    pub fn realloc(p: *mut c_void, size: u32) -> *mut c_void;
    pub fn free(p: *mut c_void);	
}

// Listed below are the five allocation functions currently required by custom
// allocators. Their signatures and symbol names are not currently typechecked
// by the compiler, but this is a future extension and are required to match
// what is found below.
//
// Note that the standard `malloc` and `realloc` functions do not provide a way
// to communicate alignment so this implementation would need to be improved
// with respect to alignment in that aspect.

#[no_mangle]
pub extern fn __rust_allocate(size: usize, _align: usize) -> *mut u8 {
    unsafe {
        let _cs = CriticalSection::new();
        malloc(size) as *mut u8
    }
}

#[no_mangle]
pub extern fn __rust_deallocate(ptr: *mut u8, _old_size: usize, _align: usize) {
    unsafe {
        let _cs = CriticalSection::new();
        free(ptr as *mut c_void)
    }
}

#[no_mangle]
pub extern fn __rust_reallocate(ptr: *mut u8, _old_size: usize, size: usize,
                                _align: usize) -> *mut u8 {
    unsafe {
        let _cs = CriticalSection::new();
        realloc(ptr as *mut c_void, size) as *mut u8
    }
}

#[no_mangle]
pub extern fn __rust_reallocate_inplace(_ptr: *mut u8, old_size: usize,
                                        _size: usize, _align: usize) -> usize {
    old_size // this api is not supported by libc
}

#[no_mangle]
pub extern fn __rust_usable_size(size: usize, _align: usize) -> usize {
    size
}
