## Functions:

- main.rs:

- basic functions:
```rust
fn main() {
   helloworld(); 
}

fn helloworld() {
    println!("Hello, World");
}
```

- passing strings to a func
```rust
fn main() {
    let name:String = String::from("NameOne");
    display(name);
}

fn display(param_name:String) {
    println!("Name: {}", param_name);
}
```