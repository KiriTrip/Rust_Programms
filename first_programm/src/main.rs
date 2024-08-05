struct Triangel {
    catet1: f64,
    catet2: f64
}

impl Triangel {
    fn find_hyp(&self) -> f64 {
        (self.catet1 * self.catet1 + self.catet2 * self.catet2).sqrt()
    }
}

fn main() {
    let tr1 = Triangel {
        catet1: 10.0,
        catet2: 8.0
    };

    let hyp = tr1.find_hyp();

    println!("{}", hyp);
}