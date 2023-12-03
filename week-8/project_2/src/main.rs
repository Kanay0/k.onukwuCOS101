// Rust Program To Find Out The Highest Years Of Experience As A Developer Through Interview

fn main() {
 //Collecting Of Information From The Person Being Interviewed    
    let mut info = String::new();
    println!("\nEnter The Name Of The Person Interviewed: ");
    std::io::stdin().read_line(&mut info).expect("Invalid Input");
    let name:String = info.trim().parse().expect("Invalid Input");

    let mut info1 = String::new();
    println!("\nEnter The Email Of The Person Interviewed: ");
    std::io::stdin().read_line(&mut info1).expect("Invalid Input");
    let email:String = info1.trim().parse().expect("Invalid Input");

    let mut info2 = String::new();
    println!("\nEnter The Address Of The Person Interviewed: ");
    std::io::stdin().read_line(&mut info2).expect("Invalid Input");
    let address:String = info2.trim().parse().expect("Invalid Input");

    let mut info3 = String::new();
    println!("\nEnter The Age Of The Person Interviewed:");
    std::io::stdin().read_line(&mut info3).expect("Invalid Input");
    let age:i32 = info3.trim().parse().expect("Invalid Input");

    let mut info4 = String::new();
    println!("\nWhich Gender: 'Male' Or 'Female'");
    std::io::stdin().read_line(&mut info4).expect("Failed Input");
    let gender:String = info4.trim().parse().expect("Failed Input");


    let mut info5 = String::new();
    println!("\nEnter The Years Of Experience: ");
    std::io::stdin().read_line(&mut info5).expect("Invalid Input");
    let year_of_experience:String = info5.trim().parse().expect("Invalid Input");
}