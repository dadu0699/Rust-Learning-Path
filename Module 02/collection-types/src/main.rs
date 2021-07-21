fn arrays() {
    // Declare array, don't specify size - compiler will infer length = 7
    // Initialize array elements using comma-separated list of values
    let days = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];
    println!("Days of the week: {:?}", days);
    // Declare array, specify length = 5, specify first element value = "0"
    // Declaration initializes every array element with value = "0"
    // Short form of: let bytes = ["0", "0", "0", "0", "0"]
    let bytes = [0; 5];
    println!("Byte buffer: {:?}", bytes);
    // Use indexing
    // Set the first day of the week
    let first = days[0];
    // Set the second day of the week
    let second = days[1];
    println!("First = {:?}, Second = {:?}", first, second);
    // Declare MUTABLE array, number of days in february changes
    let mut february = [28; 1];
    println!("February days: {:?}", february[0]);
    // Change value of element
    february[0] = 29;
    println!("February leap days: {:?}", february[0]);
    // Test out-of-bounds index
    // Set seventh day of week, use wrong index - should be 6
    // REMOVE COMMENT SLASH MARKS on next line to test
    // let seventh  = days[7]; // returns compiler error
}

fn vectors() {
    // Declare vector with three values
    let three_nums = vec![15, 3, 46];
    println!("Initial vector: {:?}", three_nums);
    // Declare vector of length = 5, specify first element value = "0"
    // Short form of: let zeroes = vec!["0", "0", "0", "0", "0"]
    let zeroes = vec![0; 5];
    println!("Zeroes: {:?}", zeroes);
    // Create empty vector, make vector mutable so it can grow and shrink
    let mut fruit = Vec::new();

    // Push some values onto the end of the vector
    // Adding values changes the type from generic to the date type of the value: String
    fruit.push("Apple");
    fruit.push("Banana");
    fruit.push("Cherry");
    println!("Fruits: {:?}", fruit);

    // Push integer value, but vector expects String (&str) type value
    // REMOVE COMMENT SLASH MARKS to test
    // fruit.push(1); // returns error
    // Pop off value at end of vector
    // We can call the pop() method from inside the println! macro
    println!("Pop off: {:?}", fruit.pop());
    println!("Fruits: {:?}", fruit);
    // Declare vector with three values
    let mut index_vec = vec![15, 3, 46];
    let three = index_vec[1];
    println!("Vector: {:?}, three = {}", index_vec, three);
    // Add 5 to the value at index 1, 5 + 3 = 8
    index_vec[1] = index_vec[1] + 5;
    println!("Vector: {:?}", index_vec);
    // Try to access the vector with an out-of-bounds index = 10
    // Program compiles, but panics and stops at the invalid expression
    // let beyond = index_vec[10];
    // println!("Vector: {:?}, {}", index_vec, beyond);
}

fn hash_maps() {
    // Use the HashMap definition from Rust standard library
    use std::collections::HashMap;
    // Declare variable as mutable so we can change keys and values
    // Both keys and values use the String type
    let mut reviews: HashMap<String, String> = HashMap::new();

    reviews.insert(
        "Ancient Roman History".to_string(),
        "Very accurate.".to_string(),
    );
    reviews.insert(
        "Cooking with Rhubarb".to_string(),
        "Sweet recipes.".to_string(),
    );
    reviews.insert(
        "Programming in Rust".to_string(),
        "Great examples.".to_string(),
    );

    // Look for specific review
    let book: &str = "Programming in Rust";
    println!("\nReview for \'{}\': {:?}", book, reviews.get(book));
    // Remove book review
    let obsolete: &str = "Ancient Roman History";
    println!("\n'{}\' removed.", obsolete);
    reviews.remove(obsolete);
    // Confirm book review removed
    println!("\nReview for \'{}\': {:?}", obsolete, reviews.get(obsolete));
}

fn main() {
    arrays();
    vectors();
    hash_maps();
}
