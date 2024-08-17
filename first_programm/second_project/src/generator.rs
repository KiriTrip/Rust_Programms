use rand::Rng;
use crate::prelude::*; // Тут мы берём всё из прилюдий, тоесть мы можем пользоваться вссем, но это не всегда удобно, потому что в больших проектах может быть много всего.
mod random_number; // Добовляем модуль

//use crate::generator::random_number::RandomNumber; // Это более длиное обращение к структуре

use random_number::RandomNumber; // Это короткое обращение к структуре на прямую

pub fn generate() -> RandomNumber { // Объявление функии публичной
    super::shared(); // Через Super можно обращаться к функциям и не только которые находяться в корне,или выше стоящем файле. Тут мы берём функцию из файла main.rs. чтобы вызвать функцию, она должна быть публичной.

    let random_number = rand::thread_rng().gen_range(LOW..=HIGH);
    RandomNumber::new(random_number)
}

pub fn other_fn() {
    println!("Game good");
}