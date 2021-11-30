// fn print_coordinates(&(x, y): &(i32, i32)) {
//     println!("location: ({},{})", x, y);
// }

// fn main2() {
//     let point = (3, 5);
//     print_coordinates(&point);
// }

// fn wildCard() {
//     let (x,y,z) = (1,2,3);
//     let [a,b,c] = [4,5,6];
//     let (i,_,_) = (7,8,9);
//     println!("xyz= {} {} {}", x, y, z);
//     println!("abc= {} {} {}", a, b, c);
//     println!("  i= {}", i);
// }

// fn int() {
//     let a = 10;
//     {
//         let mut a = 20;
//         a +=20;
//         println!("{}", a);
//     }
//     println!("{}", a);
// }

// fn array() {
//     let array = [1,2,3,4,5];
//     let slice = &array[1..2];
//     dbg!(slice);
// }

// fn increment() {
//     let mut sum = 0;
//     // 開始インデックスが含まれ、終了インデックスは含まなない
//     // m,m+1,m+2,..n-1
//     for int in 1..5 {
//         sum+= int
//     }
//     println!("sum={}",sum)
// }

// fn main() {
//     let s = String::from("Hello World");
//     // &str = 文字列のスライスの型
//     let slice = &s[0..s.len()];
//     println!("{}", slice);
// }

// fn main() {
//     let mut number = 3;
//     while number !=0  {
//         println!("{}", number);
//         number -=1;
//     };
//     println!("LIFTOFF")
// }

// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// fn main() {
//     let blue = Color(0, 0, 0);
//     let Point (x,y,z) = Point(0, 0, 0);

//     println!("{} {} {}", blue.0, blue.1, blue.2);
//     println!("{} {} {}", x, y, z);
// }

// use std::fmt;

// struct Password(String);


// // 構造体のトレイト実装するためのもの
// // トレイトのメソッドが構造体から使用できる
// impl fmt::Display for Password {
//     fn fmt() {
        
//     }
// }



// メソッドとユニット構造体実装
// selfとはメソッドを呼びたしたオブジェクトを操作できる。

// use std::fmt;
// struct Password(String);

// impl fmt::Display for Password {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.0.chars().map(|_| '*').collect::<String>())
//     }
// }

// fn main() {
//     let a = String::from("123456789");
//     println!("{}", a); // 123456789
    
//     let a = Password(String::from("123456789"));
//     println!("{}", a); // *********
// }

// struct Result {
//     width: i32,
//     height: i32
// }

// // 構造体を掛け算にする関数を実装
// impl Result {
//     fn area(&self) -> i32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let cl = Result{width: 30, height: 50};

//     println!("{}", cl.area());
// }

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self { // Self は実装している型の型エイリアス
        Self { x, y }
    }
}

fn main() {
    let a = Point::new(3., 5.);

    print!("x={}, y={}", a.x, a.y);
}

