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

- Illustration
```rust
fn main() {
    /* store names inside name */
    let mut name = String::new();
    name.push_str("NameOne");
    name.push_str("NameTwo");

    println!("{}", name);
}
```

- Illustration: Format macro
```rust
fn main() {
    let n1 = "Name".to_string();
    let n2 = "One".to_string();

    let n3 = format!("{} {}", n1, n2);
    println!("{}", n3);
}
```


```
$ cargo run
```