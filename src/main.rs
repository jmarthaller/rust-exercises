fn main() {
    // println!("Hello, Jacob!");

    // let jacob = {
    //     let name = "Jacob";
    //     let age = 28;
    //     let languages = ["Rust", "Python", "JavaScript"];
    //     (name, age, languages)
    // };

    // println!(
    //     "Hello, World! from {}, who is {} years old and knows {} languages",
    //     jacob.0,
    //     jacob.1,
    //     jacob.2.join(", ")
    // );

    // println!("\n\n\n");

    // // print!("\t\t\tWow that's {0} but also {1}", "good", "bad");

    // println!("The summation of 25 & 10 is {}", 25 + 10);

    // let mut x: f32 = 5.0;

    // println!("x is {}", x);

    // x = 10.0;

    // let y: i32 = 2;

    // println!("x + y is {}", x as i32 + y);

    // // scalar data types
    // // integers, floats, booleans, correctors

    // println!("The max value of i32 is {}", std::i32::MAX);
    // println!("The min value of i32 is {}", std::i32::MIN);
    // println!("The max value of i8 is {}", std::i8::MAX);
    // println!("The min value of i8 is {}", std::i8::MIN);

    // println!("The max value of u32 is {}", std::u32::MAX);
    // println!("The min value of u32 is {}", std::u32::MIN);
    // println!("The max value of u8 is {}", std::u8::MAX);
    // println!("The min value of u8 is {}", std::u8::MIN);

    // let status = true;

    // println!("The value of our vars is {:?}", (x, y, status));

    // // define a couple of corrector variables
    // let a: char = 'a';
    // let b: char = 'b';

    // let c: char = '\u{288A}';

    // println!("The value of my correctors is {:?}", (a, b, c));

    // let (first_number: i32, second_number: f64) = (250, 428.22);
    // let large_number: i32 = 1_000_000;
    // println!("The value of large_number is {}", large_number);

    // let x: i32 = 255;
    // println!(
    //     "The value of x is in octal is {:o} and in hexadecimal is {:X} ,binary is {:b}",
    //     x, x, x
    // );

    // // Everything in snake case

    // let n1 = 14;
    // let n2 = 14.1;
    // let n3 = n1 as f32 + n2;
    // println!("The value of n3 is {}", n3);

    // // Shadowing
    // let s: i32 = 5;
    // let s: i32 = 5 * 5;

    // println!("The value of s is {}", s);

    // let mut p: i32 = 5;
    // let p: i32 = 5 * 5;
    // println!("The value of p is {}", p);

    // let mut r: i32 = 65;
    // {
    //     r = 5;
    //     println!("The value of r in the block is {}", r);
    // }
    // println!("The value of r outside the block is {}", r);

    // // Constants
    // const PI: f32 = 3.14;
    // println!("The {1} digit of PI is {0}", PI, "hundreds");

    // COMPOND DATA TYPES
    // Strings
    // let mut s = String::from("Hello, ");
    // s.push_str("Jacob!");
    // println!("{}", s);

    // let mut growing_string: String = String::from("What a short string");
    // println!("The length of growing_string is {}", growing_string);
    // println!(
    //     "The isEmpty of growing_string is {}",
    //     growing_string.is_empty()
    // );

    // println!(
    //     "The bytes of growing_string is {}",
    //     growing_string.capacity()
    // );

    // println!(
    //     "string is in growing_string {}",
    //     growing_string.contains("string")
    // );

    // let str_len: usize = growing_string.len();
    // println!("The length of growing_string is {}", str_len);

    // let non_string: &str = "";
    // println!("The isEmpty of non_string is: {}", non_string.is_empty());

    // Tuples and Arrays
    // let tuple: (i32, f64, char) = (500, 6.4, 'a');
    // let salary = tuple.0
    // let rate = tuple.1
    // let characters = tuple.2
    // println!(
    //     "The value of tuple is {salary}, {rate}, {characters}",
    //     salary, rate, characters
    // );

    let mut number_array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of number_array is {:?}", number_array);
}
