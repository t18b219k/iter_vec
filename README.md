[![Latest version](https://img.shields.io/crates/v/iter_vec.svg)](https://crates.io/crates/iter_vec)
[![Documentation](https://docs.rs/iter_vec/badge.svg)](https://docs.rs/iter_vec)
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)
![Apache](https://img.shields.io/badge/license-Apache-blue.svg)
# iter_vec
## Description
usable for vertex and index buffer temporary storage.
## Usage
```rust
let content0 = vec![1, 2, 3, 4];
let content1 = [5, 6, 7, 8];
let content2 = [9, 0];
let iter_vec = ExactSizedIterVec::build_from_vec(vec![     
    content0.iter(),
    content1.iter(),
    content2.iter(),]);
let flat: Vec<i32> = iter_vec.copied().collect();
assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0], flat)
```
