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
}
