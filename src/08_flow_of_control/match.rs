fn main(){
    let number = 13;

    println!("Tell me about {}", number);
    match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 | 13 => println!("This is a prime!"),
        // Match an inclusive range
        13..=19 => println!("A teen"),
        // Handle rest of the cases
        _ => println!("Ain't special"),
        // TODO ^ Try commenting out this catch-all arm
    }

    let boolean = true;

    let binary = match boolean {
        // The arms of a match must cover all the possible values
        false => 0,
        true => 1,
        // TODO ^ Try commenting out this arm
    };

    println!("{} -> {}", boolean, binary);
}