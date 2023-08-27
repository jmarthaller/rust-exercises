fn main() {
    // println!("Hello, Jacob!");

    let jacob = {
        let name = "Jacob";
        let age = 28;
        let languages = ["Rust", "Python", "JavaScript"];
        (name, age, languages)
    };

    println!(
        "Hello, World! from {}, who is {} years old and knows {} languages",
        jacob.0,
        jacob.1,
        jacob.2.join(", ")
    );

    println!("\n\n\n");

    // print!("\t\t\tWow that's {0} but also {1}", "good", "bad");

    println!("The summation of 25 & 10 is {}", 25 + 10);
}
