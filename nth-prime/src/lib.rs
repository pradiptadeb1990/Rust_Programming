pub fn is_prime(value: u32) -> bool {
    // let mut status: bool;
    let upper_limit: u32 = (value as f64).sqrt() as u32;
    if value % 2 == 0 {
        false
    } else {
        for i in (3..=upper_limit).step_by(2) {
            if value % i == 0{
                return false
            }
        }
        true
    }
}
pub fn find_next_prime(num: u32) -> u32{
    let mut next_prime = num;

    loop {
        next_prime += 1;
        if is_prime(next_prime){
            break;
        }
    }
    next_prime
}
pub fn nth(n: u32) -> u32 {
    let mut prime_number:u32 = 2;

    if n !=0 {
        for _ in 1..=n {
            prime_number = find_next_prime(prime_number);
        }
    }
    prime_number
}