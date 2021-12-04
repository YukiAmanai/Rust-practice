// ジェネリックス

// fn add<T: std::ops::Add<Output=T> >(a: T, b: T) -> T {
//     a+b
// }

// struct Point<T> { x: T, y: T }

// enum Result<T,E> {
//     Ok(T),
//     Err(E),
// }

// fn main<T: std::ops::Add<Output=T> >(a: T, b :T)  -> T {
//     a+b
// }
// struct Point<T> {x: T, y: T}

// impl Point<f64> {
//     fn distance(self) -> f64 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

// enum Result<T,E> {
//     Ok(T),
//     Err(E),
// }


// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }

// let five = Some(5);
// let six = plus_one(five);
// let none = plus_one(None);

// refは値の参照を束縛することが可能？左辺に使うものとして捉えとく
// &はリソースの所有権を借用することが可能。右辺に使うものと捉えとく
// Some( &x) => println!("{}", x), // move ownership
// fn main() {
//     let a: Option<String> = Some(String::from("Hello"));
//     match a {
//         Some(ref x) => println!("{}", x),//ここで所有権移動している！
//         None => ()
//     }
//     println!("{:?}", a);
// }

// struct Account {
//     name: String,
//     pass: String
// }

// fn main() {
//     let a = Account {name: String::from("name"), pass: String::from("pass") };
//     let Account {name, pass} = a;
//     println!("{} {}", name, pass);     // borrow check!! - OK
//     println!("{} {}", a.name, a.pass); // borrow check!! - Error
// }

// struct A {
//     name: String,
//     pass: String
// }

// fn main() {
//     let a = A {
//         name: String::from("name"),
//         pass: String::from("pass")
//     };
//     let A {ref name, ref pass} = a;
//     println!("{} {}", name, pass);
//     println!("{} {}", a.name, a.pass);

// }

// let some_u8_value = Some(0u8);
// match some_u8_value {
//     Some(3) => println!("three"),
//     _ => (),
// }

// if let Some(3) = some_u8_value {
//     println!("three");
// }

let num = Some(4);

match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
}
