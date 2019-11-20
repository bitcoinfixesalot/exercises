use std::cmp::Ordering;
#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    match first_list.len().cmp(&second_list.len()) {
        Ordering::Equal if is_sub(first_list, second_list) => Comparison::Equal,
        Ordering::Less if is_sub(first_list, second_list) => Comparison::Sublist,
        Ordering::Greater if is_sub(second_list, first_list) => Comparison::Superlist,
        _ => Comparison::Unequal,
    }
}

fn is_sub<T: PartialEq>(short: &[T], long: &[T]) -> bool {
    short.is_empty() || long.windows(short.len()).any(|l| l == short)
}
