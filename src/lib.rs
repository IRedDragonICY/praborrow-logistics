//! Zero-copy buffer abstraction for raw byte streams.
//!
//! Provides `RawResource` for managing raw byte buffers without allocation overhead.
//! Uses `ManuallyDrop` to take ownership of data while exposing raw pointers.
//!
//!
//! # Safety
//! Caller is responsible for ensuring the buffer outlives all references to it.

#![no_std]

extern crate alloc;

use core::mem::ManuallyDrop;
use alloc::vec::Vec;

/// A zero-copy buffer resource representing "Hilirisasi Data" (Downstreaming Data).
/// 
/// This struct holds a raw pointer to data that is NOT owned by this struct in the
/// traditional sense (or rather, ownership is manually managed).
pub struct RawResource {
    ptr: *const u8,
    len: usize,
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

    /// Returns the raw pointer to the resource data.
    pub fn as_ptr(&self) -> *const u8 {
        self.ptr
    }

    /// Returns the length of the resource data.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns a byte slice of the resource.
    ///
    /// # Safety
    /// The caller must ensure that the returned slice does not outlive the resource
    /// or be used after the resource is dropped/invalidated.
    pub unsafe fn as_slice(&self) -> &[u8] {
        // SAFETY: We constructed ptr/len from a valid Vec in refine()
        unsafe { core::slice::from_raw_parts(self.ptr, self.len) }
    }
}
