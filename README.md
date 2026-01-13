# Praborrow Logistics

English | [Indonesia](./README_ID.md)

Zero-copy data abstraction layer for the "Hilirisasi" pipeline. Handles the refinement of raw byte streams into downstream-ready resources without unnecessary cloning or allocation overhead.

## RawResource

Provides a stable pointer/length view into a byte vector. Uses `ManuallyDrop` to ensure the underlying buffer is not deallocated while the resource is active, strictly adhering to zero-copy principles during refinement.

### Usage

```rust
use praborrow_logistics::RawResource;

let data = vec![0xDE, 0xAD, 0xBE, 0xEF];
let resource = RawResource::refine(data);

// Access via safe getters
assert_eq!(resource.len(), 4);

// Access raw bytes (unsafe)
unsafe {
    let slice = resource.as_slice();
    assert_eq!(slice[0], 0xDE);
}
```

### Safety

This crate facilitates raw pointer manipulation. The caller is responsible for ensuring the lifetime of the `RawResource` does not violate memory safety guarantees of the consumptive system.


