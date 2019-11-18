#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    // if first_list.len() == second_list.len() && is_sub(first_list, second_list) {
    //     Comparison::Equal
    // } else if first_list.len() < second_list.len() && is_sub(first_list, second_list) {
    //     Comparison::Sublist
    // } else if first_list.len() > second_list.len() && is_sub(second_list, second_list) {
    //     Comparison::Superlist
    // } else {
    //     Comparison::Unequal
    // }

    match (first_list.len(), second_list.len()) {
        (f, s) if f == s && is_sub(first_list, second_list) => Comparison::Equal,
        (f, s) if f < s && is_sub(first_list, second_list) => Comparison::Sublist,
        (f, s) if f > s && is_sub(second_list, first_list) => Comparison::Superlist,
        _ => Comparison::Unequal,
    }
}

fn is_sub<T: PartialEq>(short: &[T], long: &[T]) -> bool {
    short.is_empty() || long.windows(short.len()).any(|l| l == short)
}
