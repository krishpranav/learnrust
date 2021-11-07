## Variables:

- main.rs:

- basic 
```rust
fn main() {
    let year:i32 = 2021;

    println!("This year is: {} ", year)
}
```

- storing
```rust
fn main() {
    /* storing a variable and printing it out */
    let year:i32 = 2021;
    println!("This year is: {}", year)

    /* again storing another variable inside the defined variable */
    year = 2021;
    println!("Next year is: {}", year)
}
```

```
$ cargo run
```