fn main() {
    let array = ["a", "b", "c"];

    println!("---in order---");

    for elem in array.iter() {
        println!("{}", elem);
    }

    println!("---in reverse order---");

    for elem in array.iter().rev() {
        println!("{}", elem);
    }
}
