use std::{i8,i16,i32,i64,u8,u16,u32,u64,isize,usize,f32,f64};
use std::io::stdin;

fn main()
{
    println!("------------------");
    println!("HAIL TORVALDS!");
    let num=10;
    let mut age: i32=60;
    let hate: bool=age>num;

    println!("Num is {}",num);
    println!("Max i32 is {}",i32::MAX);
    println!("Max i8 is {}",i8::MAX);
    println!("Am i hate? {}",hate);
    println!("sqrt(60)= {}", (age as f32) .sqrt( ));
    println!("------------------");
}
