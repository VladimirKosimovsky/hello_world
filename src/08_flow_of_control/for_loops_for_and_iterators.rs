fn main() {
    // iter - This borrows each element of the collection through each iteration. 
    // Thus leaving the collection untouched and available for reuse after the loop.
    let names = vec!["Alice", "Bob", "Charlie"];

    for name in names.iter() {
        match name {
            &"Alice" => println!("There's a rustacean among us!"),
            _ => println!("Hello, {}!", name),
        }
    }

    println!("names: {:?}", names);

    // into_iter - This consumes the collection so that on each iteration 
    // the exact data is provided. Once the collection has been consumed 
    // it is no longer available for reuse as it has been 'moved' within the loop.

    let names2 = vec!["Dave", "Eve", "Frank"];

    for name in names2.into_iter() {
        match name {
            "Dave" => println!("There's a rustacean among us!"),
            _ => println!("Hello, {}!", name),
        }
    }

    // println!("names: {:?}", names2);
    // FIXME ^ Comment out this line

    /*iter_mut - This mutably borrows each element of the collection,
      allowing for the collection to be modified in place.
    */
    let mut names3 = vec!["George", "Hannah", "Ian"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "George" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names3);

}