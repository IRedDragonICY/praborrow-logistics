//! Zero-copy buffer abstraction for raw byte streams.
//!
//! Provides `RawResource` for managing raw byte buffers without allocation overhead.
//! Uses `ManuallyDrop` to take ownership of data while exposing raw pointers.
//!
//! # Safety
//! 
//! This module uses unsafe code to manage memory manually. The `RawResource` struct
//! takes ownership of a `Vec<u8>` via `ManuallyDrop`, preventing automatic deallocation.
//! The `Drop` implementation properly reconstructs the `Vec` to ensure memory is freed.
//!
//! Caller is responsible for ensuring the buffer outlives all references to it.

#![no_std]

extern crate alloc;

use core::mem::ManuallyDrop;
use alloc::vec::Vec;

/// A zero-copy buffer resource representing "Hilirisasi Data" (Downstreaming Data).
/// 
/// This struct holds a raw pointer to data with manually managed ownership.
/// When the `RawResource` is dropped, the underlying memory is properly deallocated.
///
/// # Memory Management
///
/// The `refine()` method consumes a `Vec<u8>` and stores its raw pointer and length.
/// The `Drop` implementation reconstructs the `Vec` to ensure proper deallocation.
///
/// # Example
///
/// ```
/// use praborrow_logistics::RawResource;
///
/// let data = vec![1, 2, 3, 4, 5];
/// let resource = RawResource::refine(data).expect("data should not be empty");
/// assert_eq!(resource.len(), 5);
/// // Memory is automatically freed when `resource` goes out of scope
/// ```
#[doc(alias = "PinnedBuffer")]
pub struct RawResource {
    ptr: *const u8,
    len: usize,
    // Store capacity for proper Vec reconstruction
    cap: usize,
}

// SAFETY: RawResource owns its data exclusively and the pointer is never shared
// across threads without synchronization. The data is only accessed through
// the methods on RawResource which require appropriate borrows.
unsafe impl Send for RawResource {}
unsafe impl Sync for RawResource {}

impl Drop for RawResource {
    fn drop(&mut self) {
        if !self.ptr.is_null() && self.cap > 0 {
            // SAFETY: ptr, len, and cap were created from a valid Vec<u8> in refine().
            // We stored the Vec's raw pointer, length, and capacity, with the Vec's 
            // memory not being deallocated due to ManuallyDrop. Reconstructing the Vec
            // here transfers ownership back, allowing proper deallocation when the
            // reconstructed Vec goes out of scope.
            unsafe {
                let _ = Vec::from_raw_parts(self.ptr as *mut u8, self.len, self.cap);
            }
        }
    }
}

impl RawResource {
    /// HILIRISASI DATA: Refines raw data into a downstreamable resource.
    /// 
    /// Consumes a `Vec<u8>` and takes manual ownership of its memory.
    /// The memory will be properly deallocated when this `RawResource` is dropped.
    ///
    /// # Errors
    ///
    /// Returns an error if the input data is empty, as empty buffers have no
    /// meaningful use case and could lead to null pointer issues.
    ///
    /// # Safety
    ///
    /// This method is safe to call. The internal unsafe operations are encapsulated
    /// and the `Drop` implementation ensures proper cleanup.
    pub fn refine(data: Vec<u8>) -> Result<Self, &'static str> {
        if data.is_empty() {
            return Err("Cannot refine empty data: buffer must contain at least one byte");
        }
        
        let mut domesticated = ManuallyDrop::new(data);
        Ok(Self {
            ptr: domesticated.as_mut_ptr() as *const u8,
            len: domesticated.len(),
            cap: domesticated.capacity(),
        })
    }

    /// Returns the raw pointer to the resource data.
    ///
    /// # Safety Note
    ///
    /// The returned pointer is valid only while this `RawResource` exists.
    /// Do not use the pointer after the resource has been dropped.
    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.ptr
    }

    /// Returns the length of the resource data in bytes.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the resource has zero length.
    ///
    /// Note: Due to validation in `refine()`, a successfully created
    /// `RawResource` will never be empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns a byte slice of the resource.
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// - The returned slice does not outlive the resource
    /// - The slice is not used after the resource is dropped or invalidated
    /// - No mutable access to the underlying data occurs while the slice exists
    pub unsafe fn as_slice(&self) -> &[u8] {
        // SAFETY: We constructed ptr/len from a valid Vec in refine().
        // The caller guarantees the slice won't outlive the resource.
        unsafe { core::slice::from_raw_parts(self.ptr, self.len) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_refine_success() {
        let data = alloc::vec![1, 2, 3, 4, 5];
        let resource = RawResource::refine(data).expect("should succeed");
        assert_eq!(resource.len(), 5);
        assert!(!resource.is_empty());
    }

    #[test]
    fn test_refine_empty_fails() {
        let data: Vec<u8> = alloc::vec![];
        let result = RawResource::refine(data);
        assert!(result.is_err());
    }

    #[test]
    fn test_as_slice() {
        let data = alloc::vec![10, 20, 30];
        let resource = RawResource::refine(data).expect("should succeed");
        let slice = unsafe { resource.as_slice() };
        assert_eq!(slice, &[10, 20, 30]);
    }

    #[test]
    fn test_drop_is_called() {
        // This test verifies Drop doesn't panic. In a real scenario,
        // you'd use a custom allocator to verify deallocation.
        let data = alloc::vec![1, 2, 3, 4, 5];
        let resource = RawResource::refine(data).expect("should succeed");
        drop(resource);
        // If we get here without panic/leak detector complaints, Drop works
    }
}
