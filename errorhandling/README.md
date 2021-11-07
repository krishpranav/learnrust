## Error Handling:

- basic errorhandling:
```rust
fn main() {
    /* we can store n as 13 it is a odd number */
    let n = 13;

    /* if n divided by 2 it is even or else it is a odd */
    if n % 2 == 0 {
        println!("number is even");
    } else {
        panic!("NOT_AN_EVENT");
    }
}
```

- advanced errorhandling:
```rust
/* we are using fs from std */
use std::fs::File;

fn main() {
    /* open a file called test.jpg */
    let f = File::open("test.jpg");
    
    match f {
        /* if the file founded print file found */
        Ok(f) => {
            println!("file found {:?}", f);
        },
        /* else file not found */
        Err(e) => {
            println!("file not found \n{:?}", e);
        }
    }
}
```


```
$ cargo run
```