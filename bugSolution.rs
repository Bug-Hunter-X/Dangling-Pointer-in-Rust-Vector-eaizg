fn main() {
    let mut v = vec![1, 2, 3];
    let v_clone = v.clone(); //create a copy of the vector to avoid issues
    let ptr = v_clone.as_mut_ptr();
    unsafe {
        *ptr = 4;
    }
    println!( "{:?}", v_clone);
    println!( "{:?}", v);
}