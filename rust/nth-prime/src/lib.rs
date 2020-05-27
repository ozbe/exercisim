pub fn nth(n: u32) -> u32 {
    (0..n).fold(2, |prime, _| {
        let min = prime + 1;
        (min..)
            .find(|&i| is_prime(i))
            .expect(format!("Unable to find `{}` prime", n).as_str())
    })
}

fn is_prime(n: u32) -> bool {
    if n < 2 || n % 2 == 0 {
        return false;
    } else if n == 2 {
        return true;
    } else {
        let max = (n as f64).sqrt() as u32;
        for f in (3..=max).step_by(2) {
            if n % f == 0 {
                return false;
            }
        }
    }
    true
}
