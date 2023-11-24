//Rust Program To Detri
use std::io;

fn main() {
    println!("Siblings Number Determination Program");

    println!("\nEnter the number of siblings: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed Input");
    let sibling_number: u16 = input1.trim().parse().expect("Invalid Input");

    for x in 1..=sibling_number {
        println!("\nEnter The First Name Of The Sibling: ");
        let mut firstname = String::new();
        io::stdin().read_line(&mut firstname).expect("Failed Input");

        println!("\nEnter the Age Of The Sibling: ");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Failed Input");
        let sibling_age: u32 = input2.trim().parse().expect("Invalid Input");

        if sibling_age >= 18 {
            println!("\nEnter The Martial Status: 'Single' or 'Married'");
            let mut input3 = String::new();
            io::stdin().read_line(&mut input3).expect("Failed Input");
            let martial_status: String = input3.trim().parse().expect("Invalid Input");

            if martial_status == "Single" {
                println!("\nEnter Employment Status: 'Student' or 'Worker'");
                let mut input4 = String::new();
                io::stdin().read_line(&mut input4).expect("Failed Input");
                let employment_status: String = input4.trim().parse().expect("Invalid Input");

                if employment_status == "Student" {
                    println!("\nEnter the Name Of University: ");
                    let mut input5 = String::new();
                    io::stdin().read_line(&mut input5).expect("Failed Input");

                    println!("\nEnter The Course That Student Is Studying:");
                    let mut input6 = String::new();
                    io::stdin().read_line(&mut input6).expect("Failed Input");
                }else {
                    println!("You Are A Hard-Working Citizen Of The Matrix...");
                }

            }

                if martial_status == "Married" {
                    println!( "\nDoes Sibling Have Children 'Children' or 'No Children'");
                    let mut input7 = String::new();
                    io::stdin().read_line(&mut input7).expect("Failed Input");
                    let child_status: String = input7.trim().parse().expect("Failed Input");

                    println!("\nEnter The City Where The Family Lives In:");
                    let mut input8 =String::new();
                    io::stdin().read_line(&mut input8).expect("Failed Input");

                }
            
        }else{
            println!("\nHas The Sibling Writen WAEC 'yes' or 'no'");
            let mut input9 = String::new();
            io::stdin().read_line(&mut input9).expect("Failed Input");
            let waec_status:String = input9.trim().parse().expect("Invalid Input");

            if waec_status == "yes"{
                println!("\nEnter The Secondary School Attended:");
                let mut input10 = String::new();
                io::stdin().read_line(&mut input10).expect("Failed Input");
            }else {
                println!("\nEnter Your Current Class Level");
                let mut input11 = String::new();
                io::stdin().read_line(&mut input11).expect("Failed Input");

            }
        }
    }
}
