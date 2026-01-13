# Praborrow Logistics (Bahasa Indonesia)

[English](./README.md) | Indonesia

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

// Akses raw bytes (unsafe)
unsafe {
    let slice = resource.as_slice();
    assert_eq!(slice[0], 0xDE);
}
```

### Keamanan (Safety)

Crate ini memfasilitasi manipulasi pointer mentah. Pemanggil bertanggung jawab untuk memastikan masa pakai (lifetime) dari `RawResource` tidak melanggar jaminan keamanan memori dari sistem konsumtif.
