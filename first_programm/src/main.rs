#[derive(Debug)]
struct Person {
    name: String,
    surname: String,
    age: i32,
    balance: f64
}

fn main() {
    let age = 34;
    let balance = 436.7;

    let person1 = Person {
        name: "Anna".to_string(),
        surname: "Smith".to_string(),
        age,
        balance
    };

    let person2 = Person {
        name: "Denny".to_string(),
        surname: "Pop".to_string(),
        ..person1
    };


}