## Arrays:

- main.rs

- Simple array:
```rust
fn main() {
    let array:[i32;4] = [10, 20, 30, 40];
    println!("array is {:?}", array);
}
```

- Array length
```rust
fn main() {
    let array:[i32;4] = [10, 20, 30, 40];
    println!("array is {:?}", array);
    println!("array is {:?}", array.len());
}
```

```
$ cargo run
```