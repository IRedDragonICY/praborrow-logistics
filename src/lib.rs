//! Zero-copy buffer abstraction for raw byte streams.
//!
//! Provides `RawResource` for managing raw byte buffers without allocation overhead.
//! Uses `ManuallyDrop` to take ownership of data while exposing raw pointers.
//!
//! # Safety
//! Caller is responsible for ensuring the buffer outlives all references to it.

use std::mem::ManuallyDrop;

/// A zero-copy buffer resource representing "Hilirisasi Data" (Downstreaming Data).
/// 
/// This struct holds a raw pointer to data that is NOT owned by this struct in the
/// traditional sense (or rather, ownership is manually managed).
pub struct RawResource {
    pub ptr: *const u8,
    pub len: usize,
}

impl RawResource {
    /// HILIRISASI DATA: Refines raw data into a downstreamable resource.
    /// 
    /// We take ownership of a Vec<u8>, wrap it in ManuallyDrop, and extract the pointer.
    pub fn refine(data: Vec<u8>) -> Self {
        let mut domesticated = ManuallyDrop::new(data);
        Self {
            ptr: domesticated.as_mut_ptr() as *const u8,
            len: domesticated.len(),
        }
    }
}
