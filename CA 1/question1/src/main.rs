use std::io;



fn main() {
    println!("Enter your temperature");
    let mut tempc = String::new();
    io::stdin().read_line(&mut tempc).expect("Failed to read input");
    let tempc: f64 = tempc.trim().parse().expect("Not a valid number");

// Temperature in fahrenheit
    let  tempf: f64 = (9.0/5.0) * tempc + 32.0 ;
            

//Temperature in kelvin
    let tempk: f64 = tempc + 273.15 ;


    let mut classification = "";

    if tempc > 30.0{
        classification = "hot";
    }else if tempc >= 0.0 && tempc <= 30.0 {
        classification = "normal temperature";
    }else if tempc <= 0.0 {
        classification = "freezing point"
    }else if tempc < -273.0 {
        println!("not a valid number");
    }


     println!("temperature in celsius: {} temperature in fahrenheit: {} temperature in kelvin: {} classification: {}", tempc, tempf, tempk,classification);
















}