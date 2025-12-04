fn main() {
    
    // a list of nos
    let v = vec![15, 25, 35, 45, 55];
    print_vector(v);
    // Error: `v` moved into function `print_vector`

    println!("{}",v[0]); // this line gives error
    // Error: use of moved value `v`
}

fn print_vector(x:Vec<i32>){
    println!("Inside print_vector function {:?}",x);

}
