## Enums:

- main.rs:

- basic enum
```rust
/* storing car types */
enum CarType {
    SUV,
    Sedan,
}

/* printing it out */
fn main() {
    let suv = CarType::SUV;
    let sedan = CarType::Sedan;

    println!("{:?}", suv);
    println!("{:?}", sedan);
}
```

- advanced:
```rust
/* storing car types */
enum CarType {
    SUV,
    Sedan
}

/* matching the cars */
fn print_cars(car: CarType) {
    match car {

        CarType::Sedan => {
            println!("Sedan: Medium Sized Car!!");
        },

        CarType::SUV => {
            println!("SUV: Big Sized Car!");
        },
    }
}

/* printing it out */
fn main() {
    print_cars(CarType::Sedan);
    print_cars(CarType::SUV);
}
```

```
$ cargo run
```