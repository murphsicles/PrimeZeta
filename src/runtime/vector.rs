//! Runtime support for Vector type (SIMD)
#![allow(unsafe_code)]

/// SIMD vector type (u64x8) - stored as array for now
#[repr(C)]
struct VectorU64x8 {
    data: [i64; 8]
}

/// SIMD vector type (i32x4) - stored as array for now
#[repr(C)]
struct VectorI32x4 {
    data: [i32; 4]
}

/// Create a Vector<u64, 8> value
/// 
/// # Safety
/// Returns a pointer to heap-allocated VectorU64x8
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vector_make_u64x8(
    a0: i64, a1: i64, a2: i64, a3: i64,
    a4: i64, a5: i64, a6: i64, a7: i64
) -> i64 {
    // Create array of values
    let data = [a0, a1, a2, a3, a4, a5, a6, a7];
    
    // Allocate on heap and return pointer
    let boxed = Box::new(VectorU64x8 { data });
    Box::into_raw(boxed) as i64
}

/// Create a Vector<u64, 8> with all elements equal (splat)
/// 
/// # Safety
/// Returns a pointer to heap-allocated VectorU64x8
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vector_splat_u64x8(value: i64) -> i64 {
    // Create splat value
    let data = [value; 8];
    
    // Allocate on heap and return pointer
    let boxed = Box::new(VectorU64x8 { data });
    Box::into_raw(boxed) as i64
}

/// Create a Vector<i32, 4> value
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vector_make_i32x4(a0: i64, a1: i64, a2: i64, a3: i64) -> i64 {
    // Convert i64 to i32 (truncating)
    let v0 = a0 as i32;
    let v1 = a1 as i32;
    let v2 = a2 as i32;
    let v3 = a3 as i32;
    
    // Create array of values
    let data = [v0, v1, v2, v3];
    
    // Allocate on heap and return pointer
    let boxed = Box::new(VectorI32x4 { data });
    Box::into_raw(boxed) as i64
}

/// Create a Vector<i32, 4> with all elements equal (splat)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vector_splat_i32x4(value: i64) -> i64 {
    // Convert i64 to i32 (truncating)
    let v = value as i32;
    
    // Create splat value
    let data = [v; 4];
    
    // Allocate on heap and return pointer
    let boxed = Box::new(VectorI32x4 { data });
    Box::into_raw(boxed) as i64
}

/// Free a Vector<u64, 8>
/// 
/// # Safety
/// ptr must be from vector_make_u64x8 or vector_splat_u64x8
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vector_free_u64x8(ptr: i64) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut VectorU64x8);
        }
    }
}

/// Free a Vector<i32, 4>
/// 
/// # Safety
/// ptr must be from vector_make_i32x4 or vector_splat_i32x4
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vector_free_i32x4(ptr: i64) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut VectorI32x4);
        }
    }
}