use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter()
        .fold(BTreeMap::new(), |mut map, (&score, letters)| {
            letters
                .iter()
                .map(|l| l.to_ascii_lowercase())
                .for_each(|l| {
                    map.insert(l, score);
                });
            map
        })
}
