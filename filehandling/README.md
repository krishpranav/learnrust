## Filehandling

- main.rs

- create a file and write something into it
```rust
use std::io::Write;

fn main() {
    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all("Hello World".as_bytes()).expect("write failed");
    println!("file created: data.txt\n");
}
```

- read file
```rust
use std::io::Read;

fn main(){
   let mut file = std::fs::File::open("data.txt").unwrap();
   let mut contents = String::new();
   file.read_to_string(&mut contents).unwrap();
   print!("{}", contents);
}
```

- delete file
```rust
use std::fs;
fn main() {
   fs::remove_file("data.txt").expect("could not remove file");
   println!("file is removed");
}
```