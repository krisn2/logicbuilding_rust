
pub fn factorial(num:u8)->u8 {
    if num <= 0 {
        return num;
    }
    let mut a = 1;

    for i in 1..=num  {    // 1..num + 1
        a = a * i; // a *= i
    }
    a
}

pub fn is_prime(num:u8) -> bool {
    if num <= 1{
        return false;
    }
    for i in 2..(num as f64).sqrt() as u8 + 1 {
        if num % i == 0 {
            return false;
        }
    }
    true
}

pub fn fibonacci(num:u8) -> u8 {
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
pub fn palidrome(num:u32) -> bool {
    let original_val = num.to_string();
    let reverse_val : String = original_val.chars().rev().collect();
    original_val == reverse_val
}
