use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter()
        .flat_map(|(&points, letter)| {
            letter
                .iter()
                .map(move |c| (c.to_lowercase().next().unwrap(), points))
        })
        .collect()
}
