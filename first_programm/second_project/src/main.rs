mod generator {
    use rand::Rng;
    const LOW: u8 = 1;
    const HIGH: u8 = 15;

    pub struct  RandomNumber {
        pub value: u8,
    }
    impl RandomNumber {
        pub fn new(value: u8) -> Self {
            Self {
                value
            }
        }
    }


    pub fn generate() -> RandomNumber {
        let random_number = rand::thread_rng().gen_range(LOW..=HIGH);
        RandomNumber::new(random_number)
    }
}

use generator::generate;

use generator::generate as gen;

fn main() {
    let random = generate();
    let random1 = gen();
    println!("Random number is {}", random.value);
}