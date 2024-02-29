use std::mem;

fn borrow_trial(array: &[i32]) {
    println!("First element is {} and length is {}", array[0], array.len());
}

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

    // Tuples
    let tuple = (1, "a1", false);

    println!("First value of tuple is {}", tuple.0);

    let tuple_of_tuples = ((1, 2, 3), (1, 2), 1);

    println!("Tuples are printable: {:?}", tuple_of_tuples);

    let pair = (32, true);

    let (int_var, bool_var) = pair;

    println!("Int val is {} and bool val is {}", int_var, bool_var);

    let array: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Array 1: {:?}", array);

    let array1: [i32; 5] = [0; 5];

    println!("Array 2: {:?}", array1);

    println!("Array 1 2 value: {}", array[1]);

    println!("Size of a array in bytes: {}", mem::size_of_val(&array));

    borrow_trial(&array);

    // Borrow a section of array
    borrow_trial(&array[1 .. 5]);

    // Wrong for loop
    for i in 0..array.len()+1 {
        println!("{}: {}", i, array[i]);
    }

    // Good for loop
    for i in 0..array.len()+1  {
        match array.get(i) {
            Some(val) => println!("{}: {}", i, val),
            None => println!("Out of bounds"),
        }
    }
}