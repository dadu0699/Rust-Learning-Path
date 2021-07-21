// Divide one number into another, check for a remainder, check for division by zero, print the results
// The function requires two input arguments
// - dividend: The number to be divided
// - divisor: The number to divide by
// The function returns a boolean value that indicates if the dividend is divisible
fn is_divisible_by(dividend: u32, divisor: u32) -> bool {
    // If the divisor is zero, stop execution - division by zero causes a runtime error
    if divisor == 0 {
        println!("\nError! Division by zero is not allowed.");
        // To prevent division by zero, halt execution and return to the caller
        return false;
    } else if dividend % divisor > 0 {
        println!(
            "\n{} % {} has a remainder of {}.",
            dividend,
            divisor,
            (dividend % divisor)
        );
    } else {
        println!("\n{} % {} has no remainder.", dividend, divisor);
    }

    // Create the boolean value and return it to the function caller
    // We create and send the return value in a single statement, so don't end with a semicolon
    dividend % divisor == 0
}

fn main() {
    // 12 % 4 has no remainder
    if is_divisible_by(12, 4) {
        println!("12 is evenly divisible by 4.");
    }

    // 13 % 5 has a remainder of 2
    if is_divisible_by(13, 5) {
        println!("13 is evenly divisible by 5.");
    }

    // 14 % 0 is division by zero, print an error message
    if is_divisible_by(14, 0) {
        println!("14 is evenly divisible by 0.");
    }
}
