enum CarType {
    SUV,
    Sedan
}

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

fn main() {
    print_cars(CarType::Sedan);
    print_cars(CarType::SUV);
}