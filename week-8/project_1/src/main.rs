use std::io;

fn main() {
    println!("=== Nigeria Government APS Level Checker ===");
    
    loop {
        // Show menu
        println!("\nChoose your position:");
        println!("1. Office Administrator");
        println!("2. Academic"); 
        println!("3. Lawyer");
        println!("4. Teacher");
        println!("5. Exit");
        
        // Get position choice
        let mut choice = String::new();
        println!("Enter your choice (1-5):");
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number 1-5");
                continue;
            }
        };
        
        if choice == 5 {
            println!("Goodbye!");
            break;
        }
        
        // Get years of experience
        let mut exp_input = String::new();
        println!("Enter your years of experience:");
        io::stdin().read_line(&mut exp_input).expect("Failed to read input");
        let experience: u32 = match exp_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };
        
        // Check level based on position and experience
        check_level(choice, experience);
    }
}

fn check_level(position: u32, experience: u32) {
    let position_name = match position {
        1 => "Office Administrator",
        2 => "Academic", 
        3 => "Lawyer",
        4 => "Teacher",
        _ => "Unknown"
    };
    
    println!("\n=== RESULT ===");
    println!("Position: {}", position_name);
    println!("Experience: {} years", experience);
    
    let (level, role) = match position {
        1 => office_administrator_level(experience),
        2 => academic_level(experience),
        3 => lawyer_level(experience),
        4 => teacher_level(experience),
        _ => ("Unknown".to_string(), "Unknown".to_string())
    };
    
    println!("APS Level: {}", level);
    println!("Role: {}", role);
    println!("=============");
}

fn office_administrator_level(exp: u32) -> (String, String) {
    if exp <= 2 {
        ("APS 1-2".to_string(), "Intern".to_string())
    } else if exp <= 4 {
        ("APS 3-5".to_string(), "Administrator".to_string())
    } else if exp <= 8 {
        ("APS 5-8".to_string(), "Senior Administrator".to_string())
    } else if exp <= 10 {
        ("EL1 8-10".to_string(), "Office Manager".to_string())
    } else if exp <= 13 {
        ("EL2 10-13".to_string(), "Director".to_string())
    } else {
        ("SES".to_string(), "CEO".to_string())
    }
}

fn academic_level(exp: u32) -> (String, String) {
    if exp <= 2 {
        ("APS 1-2".to_string(), "-".to_string())
    } else if exp <= 4 {
        ("APS 3-5".to_string(), "Research Assistant".to_string())
    } else if exp <= 8 {
        ("APS 5-8".to_string(), "PhD Candidate".to_string())
    } else if exp <= 10 {
        ("EL1 8-10".to_string(), "Post-Doc Researcher".to_string())
    } else if exp <= 13 {
        ("EL2 10-13".to_string(), "Senior Lecturer".to_string())
    } else {
        ("SES".to_string(), "Dean".to_string())
    }
}

fn lawyer_level(exp: u32) -> (String, String) {
    if exp <= 2 {
        ("APS 1-2".to_string(), "Paralegal".to_string())
    } else if exp <= 4 {
        ("APS 3-5".to_string(), "Junior Associate".to_string())
    } else if exp <= 8 {
        ("APS 5-8".to_string(), "Associate".to_string())
    } else if exp <= 10 {
        ("EL1 8-10".to_string(), "Senior Associate 1-2".to_string())
    } else if exp <= 13 {
        ("EL2 10-13".to_string(), "Senior Associate 3-4".to_string())
    } else {
        ("SES".to_string(), "Partner".to_string())
    }
}

fn teacher_level(exp: u32) -> (String, String) {
    if exp <= 2 {
        ("APS 1-2".to_string(), "Placement".to_string())
    } else if exp <= 4 {
        ("APS 3-5".to_string(), "Classroom Teacher".to_string())
    } else if exp <= 8 {
        ("APS 5-8".to_string(), "Snr Teacher".to_string())
    } else if exp <= 10 {
        ("EL1 8-10".to_string(), "Leading Teacher".to_string())
    } else if exp <= 13 {
        ("EL2 10-13".to_string(), "Deputy Principal".to_string())
    } else {
        ("SES".to_string(), "Principal".to_string())
    }
}