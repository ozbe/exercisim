pub fn factors(n: u64) -> Vec<u64> {
    let mut result: Vec<u64> = vec![];
    let mut remaining = n;
    let mut prime = 2;

    loop {
        if remaining % prime == 0 {
            result.push(prime);
            remaining /= prime;
        } else if let Some(next_prime) = next_prime(prime, remaining) {
            prime = next_prime;
        } else {
            break;
        }
    }

    result
}

fn next_prime(c: u64, max: u64) -> Option<u64> {
    (c + 1..=max).find(|&i| is_prime(i))
}

fn is_prime(i: u64) -> bool {
    if i == 2 {
        true
    } else if i < 2 || i % 2 == 0 {
        false
    } else {
        let max = (i as f64).sqrt() as u64;
        (3..max).step_by(2).all(|d| i % d > 0)
    }
}
