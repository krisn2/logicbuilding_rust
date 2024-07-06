fn factorial(num:u64)->u64 {
    if num <= 0 {
        return num;
    }
    let mut a = 1;
    for i in 1..=num  {    // 1..num + 1
        a = a * i; // a *= i
    }
    a
}


fn is_prime(num:u64) -> bool {
    if num <= 1{
        return false;
    }
    for i in 2..(num as f64).sqrt() as u64 + 1 {
        if num % i == 0 {
            return false;
        }
    }
    true
}

fn fibonacci(num:u64) -> u64 {
    if num <= 1{
        return num ;
    }
    let mut a = 0;
    let mut b = 1;
    for _ in 2..=num {
       let c = a + b;
        a = b;
        b = c;
    }
    b
}

fn palidrome(num:u64) -> bool {
    let original_val = num.to_string();
    let reverse_val : String = original_val.chars().rev().collect();
    original_val == reverse_val
}



fn main() {
 let number = 5 ;
 let factorial_result = factorial(number);
 let prime_result = is_prime(number);
 let fibonacci_result = fibonacci(number);
 println!("Factorial of {} is {}", number, factorial_result);
 println!("Is {} prime number {}", number, prime_result);
 println!("Fibonacci of {} is {}", number, fibonacci_result);


 let number_pali = 12321;
 let result = palidrome(number_pali);
 println!("{} is palidrome {}", number_pali, result);

}
