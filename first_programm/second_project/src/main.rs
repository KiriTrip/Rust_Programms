enum MathOperations {
    Add(f64, f64),
    Sub(f64, f64),
    Mul(f64, f64),
    Div(f64, f64)
}
fn main() {
    let mo = MathOperations::Add(18.0, 9.0);

    match mo {
        MathOperations::Add(a, b) => println!("{}", a + b),
        MathOperations::Sub(a, b) => println!("{}", a - b),
        MathOperations::Mul(a, b) => println!("{}", a * b),
        MathOperations::Div(a, b) => println!("{}", a / b)
    }
}