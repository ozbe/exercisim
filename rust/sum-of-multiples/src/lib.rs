use std::collections::HashSet;

// Two implementations.
// I'd want to run some benchmarks to see which performed better in a real environment.

// only iterate over possible numbers
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors
        .iter()
        .filter(|&&i| i > 0)
        .map(|&i| (i..limit).step_by(i as usize).collect::<Vec<_>>())
        .flatten()
        .collect::<HashSet<_>>()
        .iter()
        .sum()
}

// less memory
// pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
//   (1..limit)
//     .filter(|i| {
//       factors
//         .iter()
//         .any(|&f| f > 0 && i % f == 0)
//     })
//     .sum()
// }
