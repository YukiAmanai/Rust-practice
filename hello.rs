// fn main() {
//     let mut a = 10;           // mutable object
//     let a_ref1 = &a;          // reference
//     let a_mut_ref1 = &mut a;  // mutable reference
//     let a_mut_ref2 = &mut a;  // mutable refernece
//     *a_mut_ref2 = 20;         // assign
//     println!("{}", a);        // borrow check!! - OK
// }

fn main() {
    let mut a = 10;            // mutable object
    let a_mut_ref = &mut a;    // mutable reference
    *a_mut_ref = 20;           // dereference and assign
    println!("{}", a_mut_ref); // auto dereference
}



