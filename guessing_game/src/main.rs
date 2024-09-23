use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("¡Adivina el número!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

//    println!("El número secreto is: {secret_number}");

    
    loop {
        println!("Introduce tu intento > ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error al leer intento");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Elegiste el número {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("¡Demasiado pequeño!"),
            Ordering::Greater => println!("¡Demasiado grande!"),
            Ordering::Equal => {
                println!("¡Has acertado!");
                break; 
            }
        }
    }
}
