#[derive(Debug)]
struct Animal(i32, String, f32);
fn main() {
    let object = Animal(3, "Panda".to_string(), 13.2);

    println!("{}", object.1);
}