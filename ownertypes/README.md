## Ownertypes:

- main.rs

- basic ownership:
```rust
fn main() {
    let v1 = vec![1, 2, 3];

    let v2 = v;

    println("{:?}", v);
}
```

- advanced ownerships:
```rust
fn main(){
   let u1 = 10;
   let u2 = u1;

   println!("u1 = {}",u1);
}
```


```
$ cargo run
```