use std::io;

fn main() {
    
        let mut name = String::new();
        let mut hrs  = String::new();
        let mut extra = String::new();
        

        println!("Enter your name");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read input");

        println!("How many hours do you work?");
        io::stdin().read_line(&mut hrs).expect("Invalid input");
        let hrs: i32 = hrs.trim().parse().expect("Invalid input");

        

        println!("How many extra hours do you work?");
        io::stdin().read_line(&mut extra).expect("Invalid input");
        let extra: i32 = extra.trim().parse().expect("Invalid input");

        let hrate = hrs * 3_000.0;
        let extra = extra *4_500.0;
        let gross_salary = hrs + extra;

        if hrs > 40



        