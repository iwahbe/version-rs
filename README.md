# version-rs
Lots of code which handles other pieces of software involves version
numbers. This involves creating approximately the same object with approximately
the same 100 lines of code in multiple crates. This crate aims to avoid this
boilerplate.

Obligatory Example:
```rust
use version::Version;
let v1_2_3 = Version::new(1,2,3);
assert!(v1_2_3 == Version::from_str("1.2.3").unwrap());
assert!(v1_2_3 < Version::new(1,2,12));
assert!(v1_2_3 > Version::from((1,2,0));
let (x, y, z) = random_u16_triple();
assert!(Version::from(x,y,z), (x,y,z).into());
```

Some quick copy and past ensured that `Version` works for `u8`, `u16`, and `u32`. To use, add 
```toml
[dependencies]
version-rs = "0.1"
```
to your `Cargo.toml`. If you want to use `Version` with [serde](https://crates.io/crates/serde "Serde at crates.io"), add 
`version-rs = {version = "0.1", features = ["serde"]}` to derive `Serialize` and `Deserialize`. 
