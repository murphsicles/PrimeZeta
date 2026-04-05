//! Runtime support for array operations
#![allow(unsafe_code)]

use std::alloc::{alloc, Layout};

/// Array header structure
#[repr(C)]
struct ArrayHeader {
    len: usize,
    capacity: usize,
    // The data follows immediately after this header
}

/// Create a new array with given capacity
/// 
/// # Safety
/// Returns a pointer to array data (past the header)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn array_new(capacity: usize) -> i64 {
    if capacity == 0 {
        return 0;
    }
    
    // Calculate total size: header + capacity * size_of::<i64>()
    let header_size = std::mem::size_of::<ArrayHeader>();
    let elem_size = std::mem::size_of::<i64>();
    let total_size = header_size + capacity * elem_size;
    
    // Create layout with alignment of max(align_of::<ArrayHeader>(), align_of::<i64>())
    let align = std::mem::align_of::<ArrayHeader>().max(std::mem::align_of::<i64>());
    
    match Layout::from_size_align(total_size, align) {
        Ok(layout) => {
            let ptr = unsafe { alloc(layout) };
            if ptr.is_null() {
                return 0;
            }
            
            // Write header
            let header_ptr = ptr as *mut ArrayHeader;
            unsafe {
                (*header_ptr).len = 0;
                (*header_ptr).capacity = capacity;
            }
            
            // Return pointer to data (after header)
            let data_ptr = unsafe { ptr.add(header_size) } as *mut i64;
            data_ptr as i64
        }
        Err(_) => 0
    }
}

/// Get array length
/// 
/// # Safety
/// ptr must be a valid array data pointer returned by array_new
#[unsafe(no_mangle)]
pub unsafe extern "C" fn array_len(ptr: i64) -> i64 {
    if ptr == 0 {
        return 0;
    }
    
    // Get header pointer (data pointer minus header size)
    let data_ptr = ptr as *mut i64;
    let header_size = std::mem::size_of::<ArrayHeader>();
    let header_ptr = unsafe { data_ptr.byte_sub(header_size) } as *const ArrayHeader;
    
    unsafe { (*header_ptr).len as i64 }
}

/// Get element at index
/// 
/// # Safety
/// ptr must be a valid array data pointer, index must be < len
#[unsafe(no_mangle)]
pub unsafe extern "C" fn array_get(ptr: i64, index: i64) -> i64 {
    if ptr == 0 || index < 0 {
        return 0;
    }
    
    let data_ptr = ptr as *mut i64;
    let idx = index as usize;
    
    // Get header to check bounds
    let header_size = std::mem::size_of::<ArrayHeader>();
    let header_ptr = unsafe { data_ptr.byte_sub(header_size) } as *const ArrayHeader;
    let len = unsafe { (*header_ptr).len };
    
    if idx >= len {
        return 0;
    }
    
    unsafe { *data_ptr.add(idx) }
}

/// Set element at index
/// 
/// # Safety
/// ptr must be a valid array data pointer, index must be < len
#[unsafe(no_mangle)]
pub unsafe extern "C" fn array_set(ptr: i64, index: i64, value: i64) {
    if ptr == 0 || index < 0 {
        return;
    }
    
    let data_ptr = ptr as *mut i64;
    let idx = index as usize;
    
    // Get header to check bounds
    let header_size = std::mem::size_of::<ArrayHeader>();
    let header_ptr = unsafe { data_ptr.byte_sub(header_size) } as *const ArrayHeader;
    let len = unsafe { (*header_ptr).len };
    
    if idx >= len {
        return;
    }
    
    unsafe { *data_ptr.add(idx) = value; }
}

/// Push element to array
/// 
/// # Safety
/// ptr must be a valid array data pointer
#[unsafe(no_mangle)]
pub unsafe extern "C" fn array_push(ptr: i64, value: i64) -> i64 {
    if ptr == 0 {
        return 0;
    }
    
    let data_ptr = ptr as *mut i64;
    let header_size = std::mem::size_of::<ArrayHeader>();
    let header_ptr = unsafe { data_ptr.byte_sub(header_size) } as *mut ArrayHeader;
    
    let len = unsafe { (*header_ptr).len };
    let capacity = unsafe { (*header_ptr).capacity };
    
    if len >= capacity {
        // Array is full
        return 0;
    }
    
    // Add element
    unsafe {
        *data_ptr.add(len) = value;
        (*header_ptr).len = len + 1;
    }
    
    1 // Success
}

/// Free array
/// 
/// # Safety
/// ptr must be a valid array data pointer from array_new
#[unsafe(no_mangle)]
pub unsafe extern "C" fn array_free(ptr: i64) {
    if ptr == 0 {
        return;
    }
    
    let data_ptr = ptr as *mut i64;
    let header_size = std::mem::size_of::<ArrayHeader>();
    let header_ptr = unsafe { data_ptr.byte_sub(header_size) } as *mut ArrayHeader;
    
    // Get capacity from header
    let capacity = unsafe { (*header_ptr).capacity };
    let elem_size = std::mem::size_of::<i64>();
    let total_size = header_size + capacity * elem_size;
    
    let align = std::mem::align_of::<ArrayHeader>().max(std::mem::align_of::<i64>());
    
    if let Ok(layout) = Layout::from_size_align(total_size, align) {
        unsafe {
            std::alloc::dealloc(header_ptr as *mut u8, layout);
        }
    }
}