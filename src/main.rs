fn main() {
    let x: i32 = 5;
    let y: u32 = 5;
    let z: f64 = 5.0;

    println!("x: {}, y: {}, z: {}", x, y, z);

    let greeting = String::from("hello world");

    println!("greeting: {}", greeting);

    for i in 0..10 {
        println!("i: {}", i);
    }

    let char1 = greeting.chars().nth(0);

    //not a cleaner way to handle Option
    println!("char: {}", char1.unwrap());
}
