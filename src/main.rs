use rand::Rng;
use std::io;
use std::io::Read;
fn parol_genarot(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut parol = String::new();
    for _ in 0..length {
        let ascii = rng.gen_range(29..180);
        let character = char::from(ascii);
        parol.push(character);
    }
    parol
}
fn main() {
    let mut dlinaparola:i32 = 15;
    let parol = &parol_genarot(dlinaparola.try_into().unwrap());
    println!("пароль вот этот: {}", parol);
}
