fn main() {
    print!("This is a print without a newline appended on io::stdout");

    println!("This is a print with a newline appended on io::stdout");

    eprint!("This is a print without a newline appended on io::stderr");

    println!("This is a print with a newline appended on io::stdout");

    println!("This is the first parameter {0} and this is the second one {1}", "dog", "cat");

    // Printing numbers in different formats
    println!("Base 10: {}", 69420);
    println!("Base 2: {:b}", 69420);
    println!("Base 8: {:o}", 69420);
    println!("Base 16: {:x}", 69420);
    println!("Base 16: {:X}", 69420);

    // Printing boolean expressions 
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}