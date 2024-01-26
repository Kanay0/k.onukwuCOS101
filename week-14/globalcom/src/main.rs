use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    println!("Are you an administrator, project manager, employee, customer or vendor?");
    io::stdin().read_line(&mut input).expect(" Given Input Failed");
     input = input.trim().parse().expect(" Given Input Failed");

    
    

    if input == "administrator"{
        administrator();
    }
    if input == "project manager"{
        project_manager();
    }
    if input == "employee"{
        employee();
    }
    if input == "customer"{
        customer();
    }
    if input == "vendor"{
        vendor();
    }
    else{
        println!("\nPlease type any of the following: administrator, project manager, employee, customer or vendor");
    }
    
}

fn administrator() {
    let mut file_admin = std::fs::File::open(r"C:\Users\Tobe¬Thex.ander\OneDrive\Desktop\globacom_dbase.sql").unwrap();
    let mut contents_admin = String::new();
    file_admin.read_to_string(&mut contents_admin).unwrap();
    print!("{}", contents_admin);
}

fn project_manager() {
    let mut file_project = std::fs::File::open(r"C:\Users\Tobe¬Thex.ander\OneDrive\Desktop\project_tb.sql").unwrap();
    let mut contents_project = String::new();
    file_project.read_to_string(&mut contents_project).unwrap();
    print!("{}", contents_project);
}

fn employee() {
    let mut file_employee = std::fs::File::open(r"C:\Users\Tobe¬Thex.ander\OneDrive\Desktop\staff_tb.sql").unwrap();
    let mut contents_employee = String::new();
    file_employee.read_to_string(&mut contents_employee).unwrap();
    print!("{}", contents_employee);
}

fn customer() {
    let mut file_customer = std::fs::File::open(r"C:\Users\Tobe¬Thex.ander\OneDrive\Desktop\customers_table_tb.sql").unwrap();
    let mut contents_customer = String::new();
    file_customer.read_to_string(&mut contents_customer).unwrap();
    print!("{}", contents_customer);
}

fn vendor() {
    let mut file_vendor = std::fs::File::open(r"C:\Users\Tobe¬Thex.ander\OneDrive\Desktop\dataplan_table_tb.sql").unwrap();
    let mut contents_vendor = String::new();
    file_vendor.read_to_string(&mut contents_vendor).unwrap();
    print!("{}", contents_vendor);
}

