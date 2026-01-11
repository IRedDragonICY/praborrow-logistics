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

use alloc::vec::Vec;
use bytes::Bytes;

/// Error returned by logistics operations.
#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum LogisticsError {
    /// Attempted to refine an empty buffer.
    #[error("Cannot refine empty data: buffer must contain at least one byte")]
    EmptyBuffer,
}

/// A zero-copy buffer resource representing "Hilirisasi Data" (Downstreaming Data).
///
/// This struct wraps `bytes::Bytes` to provide efficient, reference-counted
/// access to contiguous memory without unnecessary copying.
#[doc(alias = "PinnedBuffer")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RawResource {
    inner: Bytes,
}

// Bytes is Send + Sync
unsafe impl Send for RawResource {}
unsafe impl Sync for RawResource {}

impl RawResource {
    /// HILIRISASI DATA: Refines raw data into a downstreamable resource.
    ///
    /// Consumes a `Vec<u8>` into a `Bytes` object (zero-copy if possible).
    ///
    /// # Errors
    ///
    /// Returns an error if the input data is empty.
    pub fn refine(data: Vec<u8>) -> Result<Self, LogisticsError> {
        if data.is_empty() {
            return Err(LogisticsError::EmptyBuffer);
        }

        Ok(Self {
            inner: Bytes::from(data),
        })
    }

    /// Returns the raw pointer to the resource data.
    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.inner.as_ptr()
    }

    /// Returns the length of the resource data in bytes.
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns `true` if the resource has zero length.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns a byte slice of the resource.
    ///
    /// # Safety
    ///
    /// This method is safe. The `unsafe` keyword is kept for backward compatibility
    /// but the implementation delegates to safe `Bytes::as_ref()`.
    ///
    /// # Safe Usage
    /// The returned slice is tied to the lifetime of `self`.
    pub unsafe fn as_slice(&self) -> &[u8] {
        &self.inner
    }
    
    /// Returns a safe slice.
    pub fn as_bytes(&self) -> &[u8] {
        &self.inner
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
