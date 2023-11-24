use std::io;
fn main() {
    println!("\nSelect A Number For The Given Shape:");
    println!("1.Area Of Trapezium");
    println!("2.Area Of Rhombus");
    println!("3.Area Of Parallelogram");
    println!("4.Area Of Cube");
    println!("5.Volume Of The Cylinder");

let mut select = String::new();
io::stdin().read_line(&mut select).expect("Invalid Input");
let select:String = select.trim().parse().expect("Failed To Read Shape Number");

if  select == "1"{
    //Calculate The Area Of The Trapezium
    println!("\nEnter The Height In Cm :");
    let mut height = String::new();
    io::stdin().read_line(&mut height).expect("Invalid Input");
    let height:f64 = height.trim().parse().expect("Failed Input");
    
    println!("\nEnter the First Base In Cm:");
    let  mut base1 = String::new();
    io::stdin().read_line(&mut base1).expect("Invalid Input");
    let base1:f64 = base1.trim().parse().expect("Faild Input"); 

    println!("\nEnter The Second Base In Cm:");
    let mut base2 = String::new();
    io::stdin().read_line(&mut base2).expect("Invalid Input");
    let base2:f64 = base2.trim().parse().expect("Failed Input");

    let area1 = trapezium_area(height,base1,base2);
    println!("Area Of Trapezium = {}cm^2",area1);
   
}if select == "2"{
    //To Calculate The Area Of The Rhombus
    println!("\nEnter The First Diagonal In Cm:");
    let mut diagonal1 = String::new();
    io::stdin().read_line(&mut diagonal1).expect("Invalid Input");
    let diagonal1:f64 = diagonal1.trim().parse().expect("Failed Input");

    println!("\nEnter The Second Diagonal in Cm:");
    let mut diagonal2 = String::new();
    io::stdin().read_line(&mut diagonal2).expect("Invlaid Input");
    let diagonal2:f64 = diagonal2.trim().parse().expect("Failed Input");

    let area2 = rhombus_area(diagonal1,diagonal2);
    println!("Area Of Rhombus = {}cm^2",area2);

}if select == "3"{
    //Calculate The Area Of The Parallelogram
    println!("\nEnter the base In Cm");
    let mut base = String::new();
    io::stdin().read_line(&mut base).expect("Failed Input");
    let base:f64 = base.trim().parse().expect("Failed Input");

    println!("\nEnter The Altitude In Cm");
    let mut altitude = String::new();
    io::stdin().read_line(&mut altitude).expect("Failed Input");
    let altitude:f64 = altitude.trim().parse().expect("Failed Input");

    let area3 = parallelogram_area(base,altitude);
    println!("Area of Parallelogram = {}cm^2",area3);


 }if select == "4"{
   //Calculate The Volume Of The Cylinder
    println!("\nEnter the Length of the side In Cm: ");
    let mut length_of_side = String::new();
    io::stdin().read_line(&mut length_of_side).expect("Failed Input");
    let length_of_side:f64 = length_of_side.trim().parse().expect("Failed Input");

    let area4 = cube_area(length_of_side);
    println!("Area of Cube = {}cm^2",area4);

 }if select == "5"{
   //Calculate The Volume Of The Cylinder 
   
   println!("\nEnter The Radius Of The Cylinder In Cm:");
   let mut radius = String::new();
   io::stdin().read_line(&mut radius).expect("Failed Input");
   let radius:f64 = radius.trim().parse().expect("Invalid Input");
   
   println!("\nEnter The Height Of The Cylinder In Cm:");
   let mut height = String::new();
   io::stdin().read_line(&mut height).expect("Failed Input");
   let height:f64 = height.trim().parse().expect("Invalid Input");
  
   let pi:f64 = 3.142;
   let area5 = cylinder_volume(radius,height,pi);
   println!("Volume Of Cylinder = {}cm^3",area5);
 }

    }
fn trapezium_area(height: f64, base1: f64, base2: f64) -> f64 {
    let area1 = height / 2.0 * (base1 + base2);
    area1 
}

fn rhombus_area(diagonal1: f64, diagonal2: f64) -> f64 {
    let area2 = 1.0 / 2.0 * (diagonal1 * diagonal2);
    area2
}


fn parallelogram_area(base: f64, altitude: f64) -> f64 {
    let area3 = base * altitude;
    area3
}
fn cube_area(length_of_side: f64) -> f64 {
    let area4 = 6.0 * (length_of_side).powf(2.0);
    area4
}

fn cylinder_volume(radius: f64, height: f64, pi: f64) -> f64 {
    let area5 = pi * radius.powf(2.0) * height;
    area5
}