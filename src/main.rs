pub mod number_qu;
pub mod pattern;
fn main() {
    // number_questions

    let number: u8 = 5;
    let factorial_result = number_qu::factorial(number);
    let prime_result = number_qu::is_prime(number);
    let fibonacci_result = number_qu::fibonacci(number);
    println!(" Factorial of {} is {}", number, factorial_result);
    println!(" Is {} prime number {}", number, prime_result);
    println!(" Fibonacci of {} is {}", number, fibonacci_result);

    println!("\n Let's see the pattern \n");
    // //pattern

    // // Rectangle Pattern
    let row: u8 = 8;
    let col: u8 = 5;
    pattern::rectangle_pattern(row, col);
    pattern::hollow_rectangle(row, col);

    let num:u8 = 4;
    pattern::invert_half_pyramid(num);
    pattern::half_180deg_rot(num); 

    pattern::butterfly(num);

}
