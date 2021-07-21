fn main() {
    // Loop infinite, IF you remove the break on line 8
    loop {
        // Keep printing, printing, printing...
        println!("We loop forever!");
        // On the other hand, maybe we should stop.
        break; // WARNING! This statement is critical!
    }

    // Loop sets return value with break point
    let mut counter = 1;
    // stop_loop is set when loop stops
    let stop_loop = loop {
        counter *= 2;
        if counter > 100 {
            // Stop loop, return counter value
            break counter;
        }
    };
    // Loop should break when counter = 128
    println!("\nLoop break at counter value {}.\n", stop_loop);
    // Loop while counter is less than 5
    counter = 0;
    while counter < 5 {
        println!("We loop a while...");
        counter = counter + 1;
    }
    println!(); // print empty line
    // Loop with iterator to print array values
    let big_birds = ["ostrich", "peacock", "stork"];
    for bird in big_birds.iter() {
        println!("The {} is a big bird.", bird);
    }
    println!(); // print empty line
    // Loop for a range of values
    for number in 0..5 {
        println!("{}", number * 2);
    }
}
