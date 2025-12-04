fn main() {

    let v = vec![20, 40, 60, 80];
    // vector v owns the object in heap

    let v2 = v;
    let v2_return = display(v2);
    // v2 is moved into display, but returned as v2_return

    println!("In main {:?}",v);
    // Error: use of moved value `v`
    // ownership of `v` was moved into `v2`, so `v` is no longer valid
}

fn display(v:Vec<i32>)->Vec<i32> {
    // returning same vector
    println!("inside display {:?}",v);
    return v;
}