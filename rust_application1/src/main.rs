extern crate rand;

use std::{i8,i32,f32};
use std::cmp::Ordering;
use rand::Rng;
use std::io;

fn main()
{
    println!("------------------");
    println!("HAIL TORVALDS!");
    let num=10;
    let  age: i32=60;
    let hate: bool=age>num;

    println!("Num is {}",num);
    println!("Max i32 is {}", i32::MAX);
    println!("Max i8 is {}", i8::MAX);
    println!("Am i hate? {}", hate);
    println!("sqrt(60)= {}", (age as f32) .sqrt());
    println!("------------------");

    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop
    {      
        let guess: u32 = read_int();
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => 
                        {
                            println!("You win!");
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
        let guess: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return  guess;
    }
}

