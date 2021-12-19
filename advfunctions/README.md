## Adv functions
- calling function from other files


- hello.rs:
```rust
pub fn helloworld() {
    println!("Hello, World!");
}
```

- main.rs:
```rust
mod hello;

fn main() {
    hello::helloworld();
}
```

```
$ cargo run
```