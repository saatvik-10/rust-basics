use rand::{Rng, rng};
use std::collections::HashMap;
use std::{fs, thread};

enum _Result<T, E> {
    Ok(T),
    Err(E),
}
struct User {
    name: String,
    age: u32,
    active: bool,
}

trait Summary {
    fn summarize(&self) -> String {
        return String::from("No summary available"); //default implementation in case required
    }
}

trait Fix {
    fn fix(&self) -> String {
        return String::from("No fix available"); //default implementation in case required
    }
}

struct LifetimeUser<'a> {
    name: &'a str,
}

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

    let user = User {
        name: String::from("Alice"),
        age: 30,
        active: true,
    };
    println!(
        "User: {}, Age: {}, Active: {}",
        user.name, user.age, user.active
    );

    let res = fs::read_to_string("example.txt");
    match res {
        Ok(content) => println!("File content: {}", content),
        Err(e) => println!("Error reading file: {}", e),
    }

    //if in a different function we return unwrap(), it will panic if the result is an Err and the program will crash

    let mut t_rng = rng();
    let n: u32 = t_rng.random();
    println!("Random Number is: {}", n);

    /* ADVANCE CONCEPTS */

    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("{:?}", even_filter(&vec));
    println!("{:?}", vec);

    let mut _vec2 = vec![1, 2, 3, 4, 5]; // Create a vector with initial values

    let mut users = HashMap::new();
    users.insert(String::from("Alice"), 30);
    users.insert(String::from("Bob"), 25);

    let ans = users.get("Alice");

    // println!("Alice's age: {:?}", ans); //giving an Option<type>, hv to use matcher
    match ans {
        Some(age) => println!("Alice's age: {}", age),
        None => println!("Alice not found"),
    }
    //use for(key, val) loop to iterate over HashMap

    let mut v1 = vec![1, 2, 3];

    let v1_iter = v1.iter_mut();

    for val in v1_iter {
        *val += 50;
    }
    println!("Updated vector: {:?}", v1);

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Value: {}", val);
    }
    let v1 = v1.into_iter();

    for val in v1 {
        println!("Value: {}", val);
    }
    //.iter and .iter_mut borrow the vector, while .into_iter takes ownership of the vector
    // using for loop on a vector will automatically call .into_iter() on it

    let v2 = vec![1, 2, 3];

    let v2_iter = v2.iter();
    //consuming adapter
    let sum: i32 = v2_iter.sum(); //won't be able to use v2_iter again
    println!("Sum of v2: {}", sum);

    let v2_iter = v2.iter();
    let v3_iter = v2.iter();
    //iterator adapters
    let v2_iter2 = v2_iter.map(|x| x * 2);

    for i in v2_iter2 {
        println!("v2_iter2 using map: {:?}", i);
    }

    let v3_iter3 = v3_iter.filter(|x| *x % 2 == 0);

    for i in v3_iter3 {
        println!("v3_iter3 using filter: {:?}", i);
    }

    let assign_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let ans1 = assign_vec.iter().filter(|x| *x % 2 == 0).map(|x| x * 2);
    let ans2 = assign_vec.iter().filter(|x| *x % 2 == 0).map(|x| x * 2);

    for i in ans1 {
        println!("Assignment ans is: {:?}", i)
    }

    let new_ans: Vec<i32> = ans2.collect(); //using collections
    println!("New ans is: {:?}", new_ans);

    let word = String::from("hello world");
    let sliced_word = &word[0..5]; //taking reference / borrowing a part of the string

    //if we do word.clear(), this is the situation called dangling pointer and this is where rust comes in handy.

    println!("Sliced word: {}", sliced_word);

    let word = String::from("hello world");
    let word_res = find_first_word(&word);

    println!("First word: {}", word_res);

    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4]; //taking a slice of the array
    println!("Slice: {:?}", slice);

    let user = User {
        name: String::from("Alice"),
        age: 30,
        active: true,
    };
    notify(user);
    println!("User Summary: {}", user.summarize());

    let ans;

    let str1 = String::from("hello");
    {
        let str2 = String::from("world");
        ans = longest(&str1, &str2);
    }

    println!("Longest string: {}", ans);

    let name = String::from("Alice");
    let user = LifetimeUser { name: &name };

    println!("User name: {}", user.name);

    thread::spawn(|| {
        for i in 0..10000 {
            println!("Spawned thread is: {}", i);
        }
    });

    for i in 0..10000 {
        println!("Main thread is: {}", i);
    }

    let handle = thread::spawn(|| {
        for i in 0..10000 {
            println!("Spawned thread is: {}", i);
        }
    });

    handle.join().unwrap(); //wait for the spawned thread to finish like async await

    for i in 0..10000 {
        println!("Main thread is: {}", i);
    }

    let v = vec![1, 2, 3, 4, 5];

    thread::spawn(move || {
        //move keyword to take ownership of v so that it can be used in the spawned thread bcz we could go out of scope b4 the thread starts
        println!("Spawned thread is: {:?}", v);
    });
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
    println!(
        "Capacity: {}, Length: {}, pointer: {:p}",
        s.capacity(),
        s.len(),
        s.as_ptr()
    );

    s.push_str("world");
    println!(
        "Capacity: {}, Length: {}, pointer: {:p}",
        s.capacity(),
        s.len(),
        s.as_ptr()
    );

    s.push_str("this");
    println!(
        "Capacity: {}, Length: {}, pointer: {:p}",
        s.capacity(),
        s.len(),
        s.as_ptr()
    );

    s.push_str("is");
    println!(
        "Capacity: {}, Length: {}, pointer: {:p}",
        s.capacity(),
        s.len(),
        s.as_ptr()
    );

    for _ in 0..10 {
        s.push_str("rust");
        println!(
            "Capacity: {}, Length: {}, pointer: {:p}",
            s.capacity(),
            s.len(),
            s.as_ptr()
        );
    }
}

fn takes_ownership(s: String) -> String {
    println!("Taking ownership of: {}", s);
    return s;
}

fn borrow(s: &String) {
    println!("Taking ownership of: {}", s);
}

fn updated_str(s: &mut String) {
    s.push_str("World");
}

fn even_filter(vec: &Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();
    for i in vec {
        if i % 2 == 0 {
            new_vec.push(*i);
        }
    }
    return new_vec;
}

fn find_first_word(word: &String) -> &str {
    let mut index = 0;

    for (_, i) in word.chars().enumerate() {
        if i == ' ' {
            break;
        }
        index = index + 1;
    }
    return &word[0..index];
}

impl Summary for User {
    //traits are like interface in typescript
    fn summarize(&self) -> String {
        return format!(
            "User: {}, Age: {}, Active: {}",
            self.name, self.age, self.active
        );
    }
}

impl Fix for User {}

fn notify<T: Summary + Fix>(item: T) {
    println!("Notification: {}", item.summarize());
}

fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    // rust says tell me how the liftime of output relates to the input, a generic lifetime annotation is required after which the return type of the ans will be the intersection of the lifetimes of str1 and str2
    if str1.len() > str2.len() { str1 } else { str2 }
}
