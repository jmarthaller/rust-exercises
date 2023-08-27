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

    let mut x: f32 = 5.0;

    println!("x is {}", x);

    x = 10.0;

    let y: i32 = 2;

    println!("x + y is {}", x as i32 + y);

    // scalar data types
    // integers, floats, booleans, correctors

    println!("The max value of i32 is {}", std::i32::MAX);
    println!("The min value of i32 is {}", std::i32::MIN);
    println!("The max value of i8 is {}", std::i8::MAX);
    println!("The min value of i8 is {}", std::i8::MIN);

    println!("The max value of u32 is {}", std::u32::MAX);
    println!("The min value of u32 is {}", std::u32::MIN);
    println!("The max value of u8 is {}", std::u8::MAX);
    println!("The min value of u8 is {}", std::u8::MIN);

    let status = true;

    println!("The value of our vars is {:?}", (x, y, status));

    // define a couple of corrector variables
    let a: char = 'a';
    let b: char = 'b';

    let c: char = '\u{288A}';

    println!("The value of my correctors is {:?}", (a, b, c));
}
