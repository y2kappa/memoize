## Rust `memoize` proc macro

```rust
#[memoize]
fn add(x: i32, y: i32) -> i32 {
    x * y
}

fn main() {
    for _ in 0..3 {
        println!("{:?}", add(100, 200));
    }
}
```

```
Calculated "100-200" with value 20000
20000
Loaded "100-200" from cache with value 20000
20000
Loaded "100-200" from cache with value 20000
20000
```