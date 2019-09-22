use std::cmp::Ordering;

pub fn find<T, R>(array: R, key: T) -> Option<usize>
where
    T: Ord,
    R: AsRef<[T]>,
{
    let array = array.as_ref();
    if array.is_empty() {
        return None;
    }
    let mut lenght = array.len();
    let mut base = 0;
    while lenght > 1 {
        let half = lenght / 2;
        let mid = base + half;
        let cmp = array
            .get(mid)
            .map(|element| element.cmp(&key))
            .unwrap();
        base = if cmp == Ordering::Greater { base } else { mid };
        lenght -= half;
    }
    if let Some(Ordering::Equal) = array.get(base).map(|elem| elem.cmp(&key)) {
        Some(base)
    } else {
        None
    }
}
