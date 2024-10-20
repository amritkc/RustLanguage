use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number");
        let secrete_number  = rand::thread_rng().gen_range(1..=100);
        let mut number: i32 = 0;
    loop {
         println!("Please enter the guess number");
    // println!("The Secreate Number is: {secrete_number}");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // let guess:u32 = guess.trim().parse().expect("Please Type a number");
    let guess:u32 = guess.trim().parse().expect("Enter Correct Number");
    println!("You Guessed: {guess}");
    match guess.cmp(&secrete_number) {
        Ordering::Less => {
            number = number+1;
            println!("Number is Too Small");
        }
        Ordering::Greater => {
            number = number+1;
            println!("Numebr is too big");
        }
        Ordering::Equal =>{
            println!("You Win!");
            println!("Total Number Tried: {number}");
            break;
        } 
    }
    }
   

}
