//Rust Program To Show The Multiplication

use std::io;

fn main() {
    for x in 1..13{
       
        println!("\nEnter The Number");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Invalid Input");
        let n:i64 = input.trim().parse().expect("Invalid Input");


        let multi1:i64 = n * 1;
        let multi2:i64 = n * 2;
        let multi3:i64 = n * 3; 
        let multi4:i64 = n * 4;
        let multi5:i64 = n * 5;
        let multi6:i64 = n * 6;
        let multi7:i64 = n * 7;
        let multi8:i64 = n * 8;
        let multi9:i64 = n * 9;
        let multi10:i64 = n * 10;
        let multi11:i64 = n * 11;
        let multi12:i64 = n * 12;




        println!("The Multiplication of 1 to n");
        println!("{}",multi1);
        println!("{}",multi2);
        println!("{}",multi3);
        println!("{}",multi4);
        println!("{}",multi5);
        println!("{}",multi6);
        println!("{}",multi7);
        println!("{}",multi8);
        println!("{}",multi9);
        println!("{}",multi10);
        println!("{}",multi11);
        println!("{}",multi12);



    }

}