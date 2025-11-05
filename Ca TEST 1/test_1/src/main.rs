use std::io;

fn main(){
    let mut input = String::new();
    
    println!("input in temperature values:");
    io::stdin().read_line(&mut input).expect("Not a vaid string");
    let temp:f64 = input.trim().parse().expect("Not a valid number");

    //Temperature converter
    let F = farenheit
    let C = celsius
    let K = kelvin
    
    F = (9.0/5.0) * C + 32.0;
    println!("0 dgr C = {}: ");

    if temp = < 0{
        println("it is at freezing point");
    }
    if temp = > 0 && = <31{
        println("it is within normal range");
    }
    if temp = > 30{
        println!("it is at hot temperature");
    }

}


    

