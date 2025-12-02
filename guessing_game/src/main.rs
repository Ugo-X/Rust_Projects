use std::io;

fn main() {
    println!("guess the number");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println1("the secrete number is : {secret_number}");

    println!("please input the number");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("you guessed:{guess}")
}
