fn main() {
    let x: i32 = 5;
    let y: u32 = 5;
    let z: f64 = 5.0;

    println!("x: {}, y: {}, z: {}", x, y, z);

    let greeting = String::from("hello world");

    println!("greeting: {}", greeting);

    let char1 = greeting.chars().nth(0);
    
    //not a cleaner way to handle Option
    println!("char: {}", char1.unwrap());

    //CONDITIONALS AND LOOPS
    
    for i in 0..10 {
        println!("i: {}", i);
    }

    let is_even = is_even(x);

    if is_even {
        println!("{} is even", x);
    } else {
        println!("{} is odd", x);
    }

    let sentence = String::from("hello world");
    let first_word = get_first_word(sentence);

    println!("First word: {}", first_word);

    update_str();

    let s1 = String::from("hello");
    let _s2 = s1.clone();
    println!("{}", s1);

    let mut my_string = String::from("hello");
    my_string = takes_ownership(my_string);
    println!("{}", my_string);

    let s1 = String::from("hello");
    borrow(&s1);
    println!("{}", s1);

    let mut s2 = String::from("hello");
    updated_str(&mut s2);
    print!("{}", s2);

    //IF YOU HAVE A SINGLE MUTABLE REFERENCE, YOU CANNOT HAVE ANY OTHER REFERENCES (MUTABLE OR IMMUTABLE) TO THE SAME VARIABLE  TO AVOID DATA INCONSISTENCY
}

fn is_even(x: i32) -> bool {
    return x % 2 == 0;
}

fn get_first_word(sentence: String) -> String {
    let mut ans = String::from("");
    for char in sentence.chars() {
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    } 
    return ans;
}

fn update_str() {
    let mut s = String::from("hello");
    println!("Capacity: {}, Length: {}, pointer: {:p}", s.capacity(), s.len(), s.as_ptr());

    s.push_str("world");
    println!("Capacity: {}, Length: {}, pointer: {:p}", s.capacity(), s.len(), s.as_ptr());

    s.push_str("this");
    println!("Capacity: {}, Length: {}, pointer: {:p}", s.capacity(), s.len(), s.as_ptr());

    s.push_str("is");
    println!("Capacity: {}, Length: {}, pointer: {:p}", s.capacity(), s.len(), s.as_ptr());

    for _ in 0..10 {
        s.push_str("rust");
        println!("Capacity: {}, Length: {}, pointer: {:p}", s.capacity(), s.len(), s.as_ptr());
    }

}

fn takes_ownership(s: String) -> String {
    println!("Taking ownership of: {}", s);
    return s;
}

fn borrow(s: &String) {
    println!("Taking ownership of: {}", s);
}

fn updated_str(s:&mut String) {
    s.push_str("World");
}