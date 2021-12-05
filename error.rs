/* エラーの勉強 */

// panic!
// fn main() {
//     let a: u64 = 5;
//     if a == 5{
//         println!("ok");
//     } else {
//         println!("crash!")
//     }
// }

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// use std::{fs::File, io::ErrorKind};

// fn main() {
//     let f = File::open("hello.txt");
//     let f = match f {
//         Ok(file) => file,
//         Err(ref error) if error.kind() == ErrorKind::NotFound => {
//             match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
//             }
//         },
//         Err(error) => {
//             panic!("There was a problem opening the file: {:?}", error)
//         },
//     };
// }

fn main() {
    let m  = Some(1).map(|x| x * 2);
    let m  = None.map(|x| x * 2);
    println!("{:?}", m); // Some(2)

    let a = Some(1).and_then(|x| Some(x * 2));
    println!("{:?}", a); // Some(2)
}

pub enum Option<T> {
    None,
    Some(T),
}
