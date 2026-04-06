fn main() {
    let limit: i32 = 1000; // Start with 1000 for validation
    let mut count: i32 = 0;
    
    for n in 2..limit {
        let mut is_prime: bool = true;
        for i in 2..n {
            if n % i == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            count += 1;
        }
    }
    
    println!("Prime count for limit={}: {}", limit, count); // Should print 168 for limit=1000
}