#[derive(Debug)]
struct Triangel {
    catet1: f64,
    catet2: f64
}

impl Triangel { // Методы для структуры Triangel

    fn find_hyp(&self) -> f64 { // Ищем гипотенузу
        (self.catet1 * self.catet1 + self.catet2 * self.catet2).sqrt()
    }

    fn find_area(&self) -> f64 { // Ищем периметр
        0.5 * self.catet1 * self.catet2
    }

    fn is_eq(&self, ar: f64) -> bool { // Проверяем можем ли один треугольник вложить в другой
        self.find_area() < ar
    }

    fn create_isc(cat: f64) -> Triangel { // создаём новый треугльник
        Triangel {
            catet1: cat,
            catet2: cat
        }
    }
}

fn main() {

    let tr1 = Triangel { // Создаём первый треугольник
        catet1: 6.0,
        catet2: 8.0
    };

    let tr2 = Triangel { // Создаём второй треугольник
        catet1: 3.0,
        catet2: 4.0
    };

    let result = tr2.is_eq(tr1.find_area());

    if result == true {
        println!("tr2 помещается в tr1");
    } else {
        println!("tr2 не помещается в tr1");
    }
}