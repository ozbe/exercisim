const FACTOR_SOUND_PAIRS: [(u32, &str); 3] = [(3, "Pling"), (5, "Plang"), (7, "Plong")];

pub fn raindrops(n: u32) -> String {
    let factor_sounds =
        FACTOR_SOUND_PAIRS.iter().fold(
            String::new(),
            |acc, (f, s)| {
                if n % f == 0 {
                    acc + s
                } else {
                    acc
                }
            },
        );

    if factor_sounds.is_empty() {
        n.to_string()
    } else {
        factor_sounds
    }
}
