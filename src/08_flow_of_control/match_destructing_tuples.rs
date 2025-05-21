fn main() {
    let triple = (0, -2, 3);

    println!("Tell me about {:?}", triple);
    // Match can be used to destructure a tuple
    match triple {
        // Destructure the second and third elements
        (0, y, z) => println!("First is 0, y is {:?}, z is {:?}", y, z),
        (1, ..)=> println!("First is 1, and the rest doesn't matter"),
        (..,2) => println!("The last is 2, and the rest doesn't matter"),
        (3, .., 4)  => println!("First is `3`, last is `4`, and the rest doesn't matter"),
        // `..` can be used to ignore the rest of the tuple
        _      => println!("It doesn't matter what they are"),
        // `_` means don't bind the value to a variable
    }

    let four = (-1, 2, 3, 4);
    match four {
        // First and third elements are metter
        (1, _, 3, _) => println!("First is 1, third is 3",),
        _      => println!("It doesn't matter what they are"),
    }

}