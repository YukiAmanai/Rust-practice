// fn add(a: i32, b: i32) -> i32 {
//     a+b
// }

// fn main() {
//     println!("足した値は{}",add(1,43));
// }

// fn print(&(x,y): &(i32, i32)) {
//     println!("location {} {}",x,y )
// }

// fn main() {
//     let a = (3,5);
//     print(&a);
// }

// fn main() {
//     let number = 10;
//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2")
//     } else {
//         println!("number is not divisible by 4 or 2");
//     }
// }

// fn main() {
//     let condition = true;
//     let number = if condition { 5 } else { 6 };
//     // let number = if condition { 5 } else { "six" }; Error!
//     println!("The value of number is: {}", number);
// }

// fn main() {
//     let mut counter = 0;
//     let result = loop {
//         counter+=1;
//         if counter == 10 {
//             break counter;
//         }
//     };
//     println!("The result is {}", result);
// }

// fn incrementFc(&(a,b): &(i32, i32)) {
//     println!("location {} {} ", a,b)
// }

// fn main() {
//     let fc = (1,5);
//     incrementFc(&fc);
// }
