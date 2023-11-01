    //Rust Program To Calculate The Speed Of The Cars
    use std::io;

fn main()
{
  

    let mut distance = String::new();
    let mut time = String::new(); 

    println!("Enter The Distance:  ");
    io::stdin().read_line(&mut distance).expect("Invaild Number");
    let d:f64 = distance.trim().parse().expect("Invalid Number");

    println!("Enter The Time ");
    io::stdin().read_line(&mut time).expect("Invalid Number");
    let t:f64 = time.trim().parse().expect("Failed Attempt");

    let s:f64 = (d * 1.60) / t;
    let speed = s ;
    println!("Speed Of The Car: {}", speed);
}

