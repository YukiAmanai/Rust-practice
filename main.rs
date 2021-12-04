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

fn main() {
    let a: Option<String> = Some(String::from("Hello"));
    match a {
        Some(ref x) => println!("{}", x),//ここで所有権移動している！
        // refは値の参照を束縛することができるもの？
        // Some( &x) => println!("{}", x), // move ownership
        None => ()
    }
    println!("{:?}", a);
}


// fn main() {
//     let a: Option<String> = Some(String::from("hello"));
//     match a {
//         Some(ref x) => println!("{}", x), // move ownership
//         None => ()
//     }
//     println!("{:?}", a);
// }


