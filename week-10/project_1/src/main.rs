// Create A Laptop Structure System

struct  Laptop{
    price:u32,
    quantity:u32
}
fn main() {
    //activate the structure

    let laptop1 = Laptop {
        price:650000,
        quantity:3
    };
    let laptop2 = Laptop{
        price:755000,
        quantity:3
    };
    let laptop3 = Laptop{
        price:850000,
        quantity:3,
    };

    let hp = laptop1.price * laptop1.quantity;
    let ibm = laptop2.price * laptop2.quantity;
    let toshiba = laptop3.price * laptop3.quantity;

    let totals = hp + ibm + toshiba;
    
    println!("The Total Of Your Order Is N{}",totals);
}
