fn main() {
    let logical: bool = true;

    let float: f64 = 7.4;
    let a_float = 7.6f64;
    let b_float = 7.3; 

    println!("Regular notation {0}, suffix notation {1} and implicit notation {2}", float, a_float, b_float);

    let mut mutable = 23;

    println!("Declaration: {}", mutable);

    mutable = 32;

    println!("Edit: {}", mutable);

    // Shadowing (redeclaring a variable)
    let mutable = true;
}