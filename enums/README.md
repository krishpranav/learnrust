## Enums:

- main.rs:

- basic enum
```rust
enum CarType {
    SUV,
    Sedan,
}

fn main() {
    let suv = CarType::SUV;
    let sedan = CarType::Sedan;

    println!("{:?}", suv);
    println!("{:?}", sedan);
}
```

```
$ cargo run
```