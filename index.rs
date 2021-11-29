// fn main() {
//     let mut a = 10;           // mutable object
//     let a_ref1 = &a;          // reference
//     let a_mut_ref1 = &mut a;  // mutable reference
//     let a_mut_ref2 = &mut a;  // mutable refernece
//     *a_mut_ref2 = 20;         // assign
//     println!("{}", a);        // borrow check!! - OK
// }

// fn main() {
//     let mut a = 10;            // mutable object
//     let a_mut_ref = &mut a;    // mutable reference
//     *a_mut_ref = 20;           // dereference and assign
//     println!("{}", a_mut_ref); // auto dereference
// }

// fn main() {
//     let mut a = 10;         
//     let a_ref1 = &a;
//     let a_mut_ref1 = &mut a;
//     let a_mut_ref2 = &mut a;
//     let a_ref2 = &a;
//     println!("{}", a_ref2);
// }


// 可変性とloop,breakeの練習
// fn main() {
//     let mut counter = 1;
//     loop {
//         if counter > 10 {
//             break;
//         } else {
//             println!("{}", counter);
//             counter += 1;
//         }
//     }
// }

fn main() {
    println!("何か入力してください！");

    let mut word = String::new();
    std::io::stdin().read_line(&mut word).ok();
    let answer = word.trim().to_string();

    println!("{},{}", "正解!",answer);
}
