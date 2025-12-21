# memchr-rs

Fast memchr and memchr2 implementations in Rust.

Originally extracted from [microsoft/edit](https://github.com/microsoft/edit) under the [MIT license](https://opensource.org/licenses/MIT).

## Usage

Add the following crate to your `Cargo.toml` and enable either/both the `memchr` and `memchr2` feature as needed:

```toml
[dependencies]
memchr-rs = { version = "1", features = ["memchr"] }
```

Then, use the provided APIs to perform fast byte searches:

```rust
use memchr_rs::memchr;

fn main() {
    let haystack = b"Hello, world!";
    let index = memchr(b'w', haystack, 0);
    assert_eq!(index, 7);
    println!("Found 'w' at position: {index}");
}
```

For `no_std` environments, disable the `default-features`:

```toml
[dependencies]
memchr-rs = { version = "1", default-features = false, features = ["memchr"] }
```
