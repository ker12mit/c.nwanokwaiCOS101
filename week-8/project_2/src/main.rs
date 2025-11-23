use std::io;

fn main() {
    println!("EY Nigeria - Hire the Best Developer");
    println!("=====================================\n");
    
    // Simple comparison between 2 developers
    println!("Let's compare 2 developers:");
    
    // Developer 1
    println!("\n--- Developer 1 ---");
    println!("Enter name:");
    let mut name1 = String::new();
    io::stdin().read_line(&mut name1).expect("Failed to read");
    let name1 = name1.trim();
    
    println!("Enter years of experience:");
    let mut exp1 = String::new();
    io::stdin().read_line(&mut exp1).expect("Failed to read");
    let exp1: u32 = exp1.trim().parse().expect("Enter a number");
    
    // Developer 2
    println!("\n--- Developer 2 ---");
    println!("Enter name:");
    let mut name2 = String::new();
    io::stdin().read_line(&mut name2).expect("Failed to read");
    let name2 = name2.trim();
    
    println!("Enter years of experience:");
    let mut exp2 = String::new();
    io::stdin().read_line(&mut exp2).expect("Failed to read");
    let exp2: u32 = exp2.trim().parse().expect("Enter a number");
    
    // Compare and show result
    println!("\n=== RESULT ===");
    if exp1 > exp2 {
        println!("Selected: {}", name1);
        println!("Experience: {} years", exp1);
    } else if exp2 > exp1 {
        println!("Selected: {}", name2);
        println!("Experience: {} years", exp2);
    } else {
        println!("Both have same experience: {} years", exp1);
        println!("Further evaluation needed!");
    }
    
    println!("EY Nigeria will hire this developer!");
}