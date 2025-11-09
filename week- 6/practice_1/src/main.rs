fn main() {
    let name = "chudi";    
    let uni :&str = "PAU"
    let addr:&str = "lekki in lagos"
    println!("name:{}",name );
    println!("Uninersity{},\nadrress",uni,addr );


    let dep:&'static str = "computer science";
    let school:&'static str = "sch of science and tech"
    println!("department {} \nSchool: {} ",dep ,school );





}