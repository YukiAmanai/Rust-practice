// fn print_coordinates(&(x, y): &(i32, i32)) {
//     println!("location: ({},{})", x, y);
// }

// fn main() {
//     let point = (3, 5);
//     print_coordinates(&point);
// }

fn main() {
    let a = 42;
    let ref_ref_ref_a = &a;
    let ref_a = ref_ref_ref_a;
    let b = ref_a;
    println!("{}", b === ref_a );
}
