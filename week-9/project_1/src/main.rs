use std::fs::File;
use std::io::Write;

fn main() {
    
    let mut file = File::create("drinks.txt").expect("Could not create file");

    
    let content = "
High Quality Categories of Drinks

Lager:
  - 33 Export
  - Desperados
  - Goldberg
  - Gulder
  - Heineken
  - Star

Stout:
  - Legend
  - Turbo King
  - Williams

Non-Alcoholic:
  - Maltina
  - Amstel Malta
  - Malta Gold
  - Fayrouz
";

    
    file.write_all(content.as_bytes()).expect("Could not write to file");

    println!("File created successfully!");
}