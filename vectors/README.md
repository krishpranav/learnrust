## Vectors

- basic vectors:
```rust
fn main() {
   let mut v = Vec::new();
   v.push(20);
   v.push(30);
   v.push(40);
   v.push(500);

   for i in &v {
      println!("{}",i);
   }
   println!("{:?}",v);
}
```

- advanced vectors:
```rust
use std::collections::HashSet;

fn main() {
    let mut names = HashSet::new();

    names.insert("NameOne");
    names.insert("NameTwo");
    names.insert("NameThree");

    if names.contains(&"NameOne") {
        println!("Name One Founded!");
    }
}
```