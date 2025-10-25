use std::io;

fn main() {
    
        let mut input1 = String::new();
        let mut input2 = String::new();

        println!("Are you experienced in this line of work? (y/n)");
        io::stdin().read_line(&mut input1).expect("Invalid input");
        let exp_status = input1.trim();

        println!("How old are you?");
        io::stdin().read_line(&mut input2).expect("Invalid input");
        let age: i32 = input2.trim().parse().expect("Invalid input");

        if exp_status == "y" {
            println!("You are experienced.");
            println!("{}", exp_status);
            if age >= 40 {
                println!("Your Incentive is NGN1,560,000.00");
            } else if age >= 30 && age < 40 {
                println!("Your Incentive is NGN1,480,000.00");
            } else {
                println!("Your Incentive is NGN1,300,000.00");
            }
        } else if exp_status == "n" {
            println!("You are inexperienced.");
            println!("{}", exp_status);
            println!("Your Incentive is NGN100,000.00");
        } else {
            println!("Invalid Input.");
            
        }
    
}