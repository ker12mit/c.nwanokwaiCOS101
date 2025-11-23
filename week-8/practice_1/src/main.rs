fn  main() {
    //using Vec::new()
    let v : Vec<i64> = Vec::new();

    //printing the size of vector
    println!("\n the length of Vec::new is: {}",v.len());

    //using macro
    let v = vec!["Grace","Effiong","Basil","kareem","Susan"];

    //printing the size of vector
    println!("\n The length of vec macro is: {}",v.len() );

}