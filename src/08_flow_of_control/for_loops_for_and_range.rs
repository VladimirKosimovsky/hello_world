fn main() {
    for n in 1..100 {
    // for n in 1..=100 {  
        if n % 15 == 0 {
            println!("Fizzbuzz");
        } else if n % 3 == 0 {
            println!("Fizz");
        } else if n % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", n);
        }
    }
}
