## Loops:

- basic for loop:
```rust
fn main(){
   for x in 1..11{ 
      if x==5 {
         continue;
      }
      println!("x is {}",x);
   }
}
```

- advance loop
```rust
fn main() {
    let mut x = 0;

    loop {
        x += 1;
        println!("x={}", x);

        if x == 15 {
            break;
        }
    }
}
```

- output:
```
x=1
x=2
x=3
x=4
x=5
x=6
x=7
x=8
x=9
x=10
x=11
x=12
x=13
x=14
x=15
```

```
$ cargo run
```