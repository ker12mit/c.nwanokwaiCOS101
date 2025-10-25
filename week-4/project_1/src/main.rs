use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("Please enter your quadratic variables (a, b, c):");

    io::stdin().read_line(&mut a).expect("Failed to read input");
    let a: f64 = a.trim().parse().expect("Not a valid number");

    io::stdin().read_line(&mut b).expect("Failed to read input");
    let b: f64 = b.trim().parse().expect("Not a valid number");

    io::stdin().read_line(&mut c).expect("Failed to read input");
    let c: f64 = c.trim().parse().expect("Not a valid number");

    let d = b * b - 4.0 * a * c;

    if d > 0.0 {
        let root1 = (-b + d.sqrt()) / (2.0 * a);
        let root2 = (-b - d.sqrt()) / (2.0 * a);

        println!("Discriminant = {}", d);
        println!("Two distinct real roots: {:.2} and {:.2}", root1, root2);
    } else if d == 0.0 {
        let root = -b / (2.0 * a);
        println!("Discriminant = {}", d);
        println!("One real root: {:.2}", root);
    } else {
        println!("Discriminant = {}", d);
        println!("No real roots, the roots are imaginary.");
    }
}
