# Praborrow Logistics

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

---

# Praborrow Logistics (Bahasa Indonesia)

Layer abstraksi data zero-copy untuk pipeline "Hilirisasi". Menangani pemurnian (refinement) aliran byte mentah (raw byte streams) menjadi sumber daya yang siap digunakan di hilir tanpa overhead cloning atau alokasi yang tidak perlu.

## RawResource

Menyediakan tampilan pointer/panjang yang stabil ke dalam vektor byte. Menggunakan `ManuallyDrop` untuk memastikan buffer yang mendasarinya tidak dialokasikan kembali saat sumber daya sedang aktif, dengan ketat mematuhi prinsip zero-copy selama proses pemurnian.

### Penggunaan (Usage)

```rust
use praborrow_logistics::RawResource;

let data = vec![0xDE, 0xAD, 0xBE, 0xEF];
let resource = RawResource::refine(data);

// Akses melalui getter yang aman
assert_eq!(resource.len(), 4);

// Akses byte mentah (unsafe)
unsafe {
    let slice = resource.as_slice();
    assert_eq!(slice[0], 0xDE);
}
```

### Keamanan (Safety)

Crate ini memfasilitasi manipulasi pointer mentah. Pemanggil bertanggung jawab untuk memastikan masa pakai (lifetime) dari `RawResource` tidak melanggar jaminan keamanan memori dari sistem konsumtif.

