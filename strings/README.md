## Strings:
- storing texts or variables is known as string we use the keyword ```let``` in rust

- main.rs

- basic strings
```rust
fn main() {
    /* string */
    let company:&str = "CompanyOne";
    let location:&str = "Some where in the world";

    println!("company name: {} location: {}", company, location);
}
```

- illustration
```rust
fn main() {
    /* store names inside name */
    let mut name = String::new();
    name.push_str("NameOne");
    name.push_str("NameTwo");

    println!("{}", name);
}
```

```
$ cargo run
```