extern crate rand;

use std::cmp::Ordering;
use rand::Rng;
use std::io;

fn main()
{
    println!("--------------------------------------------");
    println!("Hello Rust!");
    println!("HAIL TORVALDS!");
    println!("--------------------------------------------");

    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut tries=0;

    loop
    {      
        let guess: u32 = read_int();
        println!("You guessed: {}", guess);
        tries+=1;
        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => 
                        {
                            println!("You win!");
                            println!("Tries: {}",tries);
                            break;
                        }
            }
    }
}

fn read_int() -> u32
{
    loop 
    {     
       println!("Please input your guess.");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => continue,
        };
    }
}

