use std::io::Read;

fn main(){
    let mut file = std::fs::File::open(r"C:\Users\Tobe¬Thex.ander\OneDrive\المستندات\Computer Science 101\k.onukwuCOS101\week-14\globacom\staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);
}
