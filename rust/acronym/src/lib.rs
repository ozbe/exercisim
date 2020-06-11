pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c.is_whitespace() || c == '-')
        .flat_map(|w| {
            let mut chars = w.chars().skip_while(|c| !c.is_alphabetic());
            std::iter::once(chars.next())
                .flatten()
                .map(|c| c.to_ascii_uppercase())
                .chain(
                    chars
                        .skip_while(|c| c.is_uppercase())
                        .filter(|c| c.is_uppercase()),
                )
        })
        .collect::<String>()
}
