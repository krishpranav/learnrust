use std::fs::File;

fn main() {
    let f = File::open("test.jpg");
    
    match f {
        Ok(f) => {
            println!("file found {:?}", f);
        },
        Err(e) => {
            println!("file not found \n{:?}", e);
        }
    }
}