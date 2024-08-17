mod shapes;

fn main() {
    println!("Circle area: {}", shapes::Circle { radius: 7. }.get_area());
    println!("Rectangle area: {}", shapes::Rectangle::new(5., 4.).get_area());
}