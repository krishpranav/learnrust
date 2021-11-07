fn main() {
    for x in 1..11 {
        if x == 2 {
            continue;
        }
        println!("x is {}", x);
    }
}