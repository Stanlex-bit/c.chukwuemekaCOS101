use std::io;

fn main(){

    let mut input1 = String::new();
    let mut input2 = String::new();
    let input3 = String::new();

    println!("Enter the value of a:");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f64 = input1.trim().parse().expect("Not a valid number");

    println!("Enter the value of b:");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f64 = input2.trim().parse().expect("Not a valid number");

    println!("Enter the value of c:");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let c:f64 = input3.trim().parse().expect("Not a valid string");

    // Find the discriminant
    let discriminant = b * b - 4.0 * a * c;

    //Determine nature of roots
    if discriminant > 0.0
    {
    let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
    let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
    println!("There are two distinct roots");
    println!("Root 1 = :{} ", root1);
    println!("Root 2 = :{} ", root2);
    }

    else if discriminant == 0.0
    {
        let root = -b / (2.0 * a);
        println!("There is one root");
        println!("Root = :{} ", root);
    }

    else {
        println!("There are no real roots");
    }
}
    

