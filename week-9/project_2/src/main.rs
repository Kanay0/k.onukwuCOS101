//Rust Program To Store Students Data From The Student's Database 
use std::io::Write;

fn main() {
    println!("Student Detail Create And Display Program");
     
    let student_name = vec!["Oluchi Mordi","Adams Aliyu","Shania Bolade","Adekunle Gold","Bianca Edemoh"];
    let matric_no = vec!["ACC10211111","ECO10110101","CSC10328828","EEE11020202","MEE10202001"];
    let dept = vec!["Accounting","Economics","Computer","Electrical","Mechanical"];
    let level = vec![300,100,200,200,100];
    let mut file = std::fs::File::create("Details Of Students.txt").expect("Create Failed");

    for i in 0..level.len(){

        println!("Name:{}\nMatric NO:{}\nDepartment:{}\nLevel:{}\n",student_name[i],matric_no[i],dept[i],level[i] );

           file.write_all(format!("{}\t", student_name[i]).as_bytes()).expect("Write Failed");
           file.write_all(format!("{}\t", matric_no[i]).as_bytes()).expect("Write Failed");
           file.write_all(format!("{}\t", dept[i]).as_bytes()).expect("Write Failed");
           file.write_all(format!("{}\n", level[i]).as_bytes()).expect("Write Failed");
            println!("DATA WRITTEN INTO FILE");
    

         
    }

   
}
