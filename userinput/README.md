## Userinput:

- main.rs

- userinput
```rust
fn main() {
    /* line string */
    let mut line = String::new();

    /* enter your name */
    println!("Enter your name: ");

    /* store the user input under b1 and pass it to line */
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    
    /* print the user input */
    println!("Hello: {}", line);
}
```

```
$ cargo run
```