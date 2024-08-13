enum Person {
    Adult,
    Underage
}

fn main() {
    let person = Person::Adult;

    match person {
        Person::Adult => println!("Заходи"),
        Person::Underage => print!("Пошёл нахуй")
    }
}