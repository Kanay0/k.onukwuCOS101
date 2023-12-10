
use std::io::Write;

fn main() {

   let lager = "\nLager\n33 Export\nDespersdos\nGoldberg\nGulder\nHeinkein\nStar\n";
   let stout = "\nStout\nLegend\nTurboKing\nWilliams\n";
   let non_alcholic = "\nNon-Alcholic\nMaltina\nAmstel Malta\nMalta Gold\nFavourz\n";

   let mut filer = std::fs::File::create("brewerberry.txt").expect("create failed");

   filer.write_all("Welcome To The Brewery Creation Control\n".as_bytes()).expect("write failed");
   filer.write_all(lager.as_bytes()).expect("write failed");
   filer.write_all(stout.as_bytes()).expect("write failed");
   filer.write_all(non_alcholic.as_bytes()).expect("Write failed");
   println!("\nDATA WRITTEN TO FILE");

     }
