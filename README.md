# RC4

An implementation of the RC4 (also sometimes called ARC4) stream cipher.

```toml
[dependencies]
rc4 = { git = "https://github.com/rsdump/rc4" }
```

- [Example](#Example)

# Example

rc4 is dead simple to use:

```rust
use std::fs;
use std::io;

fn main() {
    let src = "/tmp/src";
    let dst = "/tmp/dst";
    let k = "secret";

    let r = fs::File::open(src).unwrap();
    let mut r = rc4::Reader::new(r, k.as_bytes()).unwrap();
    let mut w = fs::File::create(dst).unwrap();
    // Awesome! r implements io::Read, so it works well with the standard library.
    io::copy(&mut r, &mut w).unwrap();
}
```
