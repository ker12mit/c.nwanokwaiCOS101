use std::io;

fn main() {
    // Display the menu
    println!("Welcome to Our Restaurant!");
    println!("==========================");
    println!("| Menu                   | Price    |");
    println!("|-------------------------|----------|");
    println!("| P = Poundo Yam/Edinkaiko Soup | N3,200  |");
    println!("| F = Fried Rice & Chicken      | N3,000  |");
    println!("| A = Amala & Ewedu Soup        | N2,500  |");
    println!("| E = Eba & Egusi Soup          | N2,000  |");
    println!("| W = White Rice & Stew         | N2,500  |");
    println!("==========================\n");

    // Variables to store order total
    let mut total = 0;
    let mut ordering = true;

    // Take orders
    while ordering {
        println!("What would you like to order?");
        println!("Enter P, F, A, E, W or 'Q' to finish:");
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        
        // Convert input to uppercase and get first character
        let choice = choice.trim().to_uppercase();
        let first_char = choice.chars().next().unwrap_or(' ');
        
        match first_char {
            'P' => {
                println!("Added: Poundo Yam/Edinkaiko Soup - N3,200");
                total += 3200;
            },
            'F' => {
                println!("Added: Fried Rice & Chicken - N3,000");
                total += 3000;
            },
            'A' => {
                println!("Added: Amala & Ewedu Soup - N2,500");
                total += 2500;
            },
            'E' => {
                println!("Added: Eba & Egusi Soup - N2,000");
                total += 2000;
            },
            'W' => {
                println!("Added: White Rice & Stew - N2,500");
                total += 2500;
            },
            'Q' => {
                ordering = false;
                println!("Finishing your order...");
            },
            _ => {
                println!("Invalid choice! Please enter P, F, A, E, W or Q.");
            }
        }
        
        if ordering {
            println!("Current total: N{}\n", total);
        }
    }

    // Check if any items were ordered
    if total == 0 {
        println!("No items were ordered. Thank you for visiting!");
        return;
    }

    // Apply discount if total is more than 10,000
    let discount;
    let final_total;
    
    if total > 10000 {
        discount = (total as f32 * 0.05) as i32;
        final_total = total - discount;
        println!("\nYour total is over N10,000! You get a 5% discount!");
        println!("Subtotal: N{}", total);
        println!("Discount: N{}", discount);
        println!("Final Total: N{}", final_total);
    } else {
        discount = 0;
        final_total = total;
        println!("\nTotal: N{}", final_total);
    }

    println!("\nThank you for your order! Please come again!");
}