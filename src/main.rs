use std::collections::HashMap;

fn main() {

    // unsigned int is only positive and 2^n range
    let a: u8 = 10;
    // unsigned int is both
    let b:i8 = 127;
    println!("Hello, world!, {} {}", a, b);

    let arr: [u8; 3] = [1,2,3];
    println!("index: {}, length: {}", arr[0], arr.len());


    let tuple: (u8, bool, f32) = (5, true, 2.1);
    println!("{} {} {}",tuple.0, tuple.1, tuple.2);

    println!("{}", is_even(5));

    let num =5;
    let mut num2 =8;
    // num =3; => not possible, not mutable
    num2 = 4;
    println!("{} {}", num, num2);

    let arr2 = [0, 1, 2, 3]; // Array is a reference cause we are pointing to the address in memory where the array is located
    let slice = &arr2[1 .. 3]; // [1,2] & is required because we don't know the length at compile time

    borrowing_slice(arr2, slice);

    let str =  "Hello 1"; // string literal, reference to a space in memory
    let mut string: String = String::from("Hello 2"); // String object

    let stringslice = &string[ .. 6];
    slice.len();

    string.push('1');
    string.push_str("again");
    string = string.replace("Hello", "Hey");
    println!("{}", string);

    let var = 3;
    if var > 0 {
        println!("greater then 0")
    } else {
        println!("not greater then 0")
    }

    for i in 0..5 {
        println!("{}", i)
    }

    let mut wil = 0;
    while wil< 4 {
        println!("{}", wil);
        wil+= 1;
        if wil ==3 {
            println!("Exit");
            break
        }
    }

    let mat = 5;
    match mat {
        0 => println!("0"),
        1 | 2 => println!("1,2"),
        3..=5 => println!("3,4,5"),
        _ => println!("Default")
    }
    let namebird = String::from("Tucan");
    let tucan = Bird { name: namebird, attack: 5};
    tucan.print_name();
    println!("{} {}",tucan.can_fly(), tucan.is_animal());

    let a: MyEnum = MyEnum::A;
    let b: MyEnum = MyEnum::B(5);
    let c: MyEnum = MyEnum::C{x:10, y:20};
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);

    if let MyEnum::B(val) = b {
        println!("{}", val);
    }

    // vector is dynamic array, still need to add mut to push or remove values
    let mut vec: Vec<i64> = vec![1,2,3,4,5];
    vec.len();
    vec[0];
    vec.push(6);
    vec.remove(0);
    println!("{:?}", vec);

    // Hash maps
    let mut map = HashMap::new();
    map.insert(0, "hello");
    map.insert(1, "hi2");
    println!("{:?}", map);

    match map.get(&0) {
        Some(str) => println!("{}", str),
        _ => println!("Doesn't exist in map")
    }

    // returns option
    match map.get(&2) {
        Some(str) => println!("{}", str),
        _ => println!("Doesn't exist in map")
    }

    map.remove(&0);
    println!("{:?}", map);

    match map.get(&0) {
        Some(str) => println!("{}", str),
        _ => println!("Doesn't exist in map")
    }

    // Options => when we want to get a value from Hashmap
    // Option always returns None - to indicate a failure-  or Some(Value) - a tuple struct that wraps a value with type T
    let divide1: Option<i32> = divide(4,2);
    let divide2: Option<i32> = divide(2,3);

    // Unwrapping a Some variant will extract the value wrapped
    println!("{:?} unwraps to {}", divide1, divide1.unwrap());

    // Unwrapping a None variant will panic! => Rust version of exception
    // println!("{:?} unwraps to {}", divide2, divide2.unwrap());

    // Result
    let divideRes = divideResult(4,2);
    // match divideRes {
    //     Ok(v) => println!("{}", v),
    //     Err(v) => println!("{:?}", v)
    // }

    if divideRes.is_ok() {
        println!("{}", divideRes.unwrap());
    }

    // println!("{}", divideRes.unwrap_or(100)); => will print 100
    // implement to catch exception:
    // let res = divideRes.expect("We crashed");
    // println!("{}", res);


}

#[derive(Debug)]
enum MyError {
    Error1
}

// Err, an enum that contains an error Code
// Ok(value), wrapper that contains a value
fn divideResult(dividend: i32, divisor: i32) -> Result<i32, MyError> {
    if dividend % divisor != 0 {
        Err(MyError::Error1)
    } else {
        Ok(dividend/divisor)
    }
}


fn divide(dividend: i32, divisor: i32) -> Option<i32> {
    if dividend % divisor != 0 {
        None
    } else {
        Some(dividend/divisor)
    }
}

#[derive(Debug)]
enum MyEnum {
    A,
    B(i32),
    C {x: i32, y: i32}
}


trait Animal {
    fn can_fly(&self) -> bool;
    fn is_animal(&self) -> bool {
        true
    }
}

impl Animal for Bird {
    fn can_fly(&self) -> bool {true}
    fn is_animal(&self) -> bool {
        false
    }
}

struct Bird {
    name: String,
    attack: u64
}

impl Bird {
    fn print_name(&self) {
        println!("{}", self.name);
    }
}

pub fn is_even(num: u8) -> bool {
    let digit: u8 = num % 2;
    digit == 0 //return can be removed if we don't put a semicolon
}

fn borrowing_slice(arr: [u8; 4], slice: &[u8]) {
    println!("{:?}", arr);
    println!("{:?}", slice);
    println!("length: {}", slice.len());
    println!("{} {}", slice[0], slice[1]);
}
