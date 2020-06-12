pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    // Add 1 to account for zero based index
    let mut a: Vec<_> = std::iter::repeat(true).take((upper_bound + 1) as usize).collect();

    for i in 2..=(upper_bound as f64).sqrt() as u64 {
      if let Some(true) = a.get_mut(i as usize) {
        for n in 0..upper_bound {
          let j = i.pow(2) + n * i;

          if j > upper_bound {
            break;
          }

          if let Some(aj) = a.get_mut(j as usize) {
            *aj = false;
          }

        }
      }
    }

    a.iter()
      .enumerate()
      .skip(2) // skip 0 & 1
      .filter(|(_, p)| **p)
      .map(|(i, _)| i as u64) 
      .collect()
}


/*
less optimal but more fluent

const LOWER_BOUND: u64 = 2;

let mut primes: Vec<_> = (LOWER_BOUND..=upper_bound).map(|n| Some(n)).collect();

for i in LOWER_BOUND..=(upper_bound as f64).sqrt() as u64 {
    (i..=upper_bound).step_by(i as usize).skip(1).for_each(|p| {
        if let Some(v) = primes.get_mut((p - LOWER_BOUND) as usize) {
            v.take();
        }
    })
}

primes.into_iter().flatten().collect()
*/