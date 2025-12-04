fn main(){

    let v = vec![101, 250, 330, 400];
    // vector v owns the object in heap  

    //only a single variable owns the heap mempry at any given time
    let v2 = v;
    // Error: ownership of `v` moved to `v2`

    //Rust is very smart in terms of memory access, so it detects a race condition
    //as two variables point to the same heap  

    println!("{:?}",v);
    // Error: use of moved value `v`
}


