pub fn build_proverb(list: &[&str]) -> String {
    match list {
        [] => String::new(),
        [first, ..] => {
            let mut proverb = list
                .iter()
                .zip(list.iter().skip(1))
                .map(|(w1, w2)| format!("For want of a {} the {} was lost.", w1, w2))
                .collect::<Vec<_>>();
            proverb.push(format!("And all for the want of a {}.", first));
            proverb.join("\n")
        }
    }
}
