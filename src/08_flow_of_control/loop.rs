fn main() {
    let mut count = 0u32;
    println!("Let's count until infinity!");

    // Infinite loop

    loop {
        count += 1;

        if count == 3 {
            println!("three");

            continue;
        }
        println!("{}", count);

        if count == 5 {
            println!("That's eneough");

            break;
        }
    }
}
