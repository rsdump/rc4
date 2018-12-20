# Rust-RC4

An implementation of the RC4 (also sometimes called ARC4) stream cipher.

```
[dependencies]
rc4 = { git = "https://github.com/mohanson/rust-rc4" }
```


# Example

rust-rc4 is dead simple to use:

```rust
extern crate rc4;

use std::fs;
use std::io;

fn main() {
    let src = "/tmp/src";
    let dst = "/tmp/dst";
    let k = "secret";

    let r = fs::File::open(src).unwrap();
    let mut r = rc4::Reader::open(r, k.as_bytes()).unwrap();
    let mut w = fs::File::create(dst).unwrap();
    // Awesome! r implements io::Read, so it works well with the standard library.
    io::copy(&mut r, &mut w).unwrap();
}
```
