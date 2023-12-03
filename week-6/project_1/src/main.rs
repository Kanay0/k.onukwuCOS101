 // Rust Program To Determine The Eligibilty To Vote For The Representatives Of The Student Council
use std::io;

fn main() {
  for x in 1..151{
    
    println!("Student Council Voting System");
  let mut input1 = String::new();
  let mut input2 = String::new();
  let mut input3 = String::new();
  

  

  println!("\nEnter Candidate Status 'course rep' or 'not course rep' ");
  io::stdin().read_line(&mut input1).expect("Not a Valid Input");
  let candidate_status:String = input1.trim().parse().expect("Invalid Input");

  println!("\nEnter The CGPA: ");
  io::stdin().read_line(&mut input2).expect("Not a Valid Input");
  let cpa:f64 = input2.trim().parse().expect("Invalid Input");


  println!("\nEnter Candidate Level:");
  io::stdin().read_line(&mut input3).expect("Not a Valid Input");
  let candidate_level:f32 = input3.trim().parse().expect("Invalid Input");

  

  //input the information from the candiate

  println!("\nEnter The Name: ");
  let mut name = String::new();
  io::stdin().read_line(&mut name).expect("Invalid Name");
  
  
  println!("\nEnter the Email: ");
  let mut email = String::new();
  io::stdin().read_line(&mut email).expect("Invalid Email");

  println!("\nEnter the Department:");
  let mut department = String::new();
  io::stdin().read_line(&mut department).expect("Invalid Department");

  println!("\nEnter the State of Origin: ");
  let mut state_of_orgin = String::new();
  io::stdin().read_line(&mut state_of_orgin).expect("Invalid Location");


// Validation Of Eligibilty To Vote 

  if candidate_status == "course rep" && cpa >= 4.0 && candidate_level > 100.0 {
  println!("You Are Eligible To Vote.");
  println!("{}",name);
  println!("{}",email);
  println!("{}",department);
  println!("{}",state_of_orgin );

  } else{
    println!("You are not Eligible To Vote");
 } 
 println!("Canidate count {} ", x);
 println!("");//just an empty line





  }



 }
 
