# simple-derive-macro-builder

# examples

See `examples/point.rs`

```rust
use proc_builder_pattern::Builder;

#[derive(Builder)]
struct Point {
    x: usize,
    y: usize,
}
```

will be expand to

```rust
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn x(self, val: usize) -> Self {
        Self { x: val, ..self }
    }
}

impl Point {
    fn y(self, val: usize) -> Self {
        Self { y: val, ..self }
    }
}
```
