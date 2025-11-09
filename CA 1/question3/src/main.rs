use std::io;

fn main() {

    println!("Welcome to teccna bookstore");
    println!("we have the following books");

    println!("code: r , title : rust for beginners , price: 15_000");
    
    println!("code: a , title : AI basics , price: 12_500");
       
    println!("code: d , title : Data structures in rust , price: 20_500");
    
    println!("code: n , title : Networking basics , price: 18_000");

    let r:f64 = 15_000.0;
    let a:f64 = 12_500.0;
    let d:f64 = 20_500.0;
    let n:f64 = 18_000.0;

 
    println!("what  books you want to buy");
    let mut code = String::new();
    io::stdin().read_line(&mut code).expect("Failed to read input");
     let code :f32 = code.trim().parse().expect("not a number");

     

    println!("Enter the number of books you want to buy");
    let mut nb = String::new();
    io::stdin().read_line(&mut nb ).expect("Failed to read input");
    let nb :f32 = nb.trim().parse().expect("not a number");


    let price = code * nb;
    let mut dprice = price * 10.0 / 100.0 ; 
    if nb >= 3.0{
      dprice = price * 10.0 / 100.0 ;  
     println!("discounted price: {}",dprice);   

  



    }else if dprice <  3.0{
        println!("no discount");
    } 
    
   
    println!("the total price of your books is {}",price );
     


}







