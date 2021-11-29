fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("location: ({},{})", x, y);
}

fn main2() {
    let point = (3, 5);
    print_coordinates(&point);
}

fn wildCard() {
    let (x,y,z) = (1,2,3);
    let [a,b,c] = [4,5,6];
    let (i,_,_) = (7,8,9);
    println!("xyz= {} {} {}", x, y, z);
    println!("abc= {} {} {}", a, b, c);
    println!("  i= {}", i);
}

fn int() {
    let a = 10;
    {
        let mut a = 20;
        a +=20;
        println!("{}", a);
    }
    println!("{}", a);
}

fn array() {
    let array = [1,2,3,4,5];
    let slice = &array[1..2];
    dbg!(slice);
}

fn increment() {
    let mut sum = 0;
    // 開始インデックスが含まれ、終了インデックスは含まなない
    // m,m+1,m+2,..n-1
    for int in 1..5 {
        sum+= int
    }
    println!("sum={}",sum)
}

fn main() {
    let s = String::from("Hello World");
    // &str = 文字列のスライスの型
    let slice = &s[0..s.len()];
    println!("{}", slice);
}
