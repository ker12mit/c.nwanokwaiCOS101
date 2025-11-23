 fn main() {
    let v = vec!['c','o','m','p','u','t','e','r'];

    let mut input1 = String::new();


    println!("enter an index value btw (0-7)");
    std::io::stdin().read_line(&mut input1).expect("failed to read input");

    //index is the non negative value which is smaller than the size of the vector
    let index:usize = input1.trim().parse().expect("invalid input");

    //getting value at given index value
    let ch: char = v[index];    

    println!("{} is the charcater for index [{}]\n",ch,index)  ;








}