use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut result: HashSet<[u32; 3]> = HashSet::new();
    for a in 1..sum {
        for b in a..sum {
            let candidate = a * a + b * b;
            let root = (candidate as f32).sqrt() as u32;

            if root * root == candidate && a + b + root == sum {
                result.insert([a, b, root]);
            }
        }
    }
    result
}
