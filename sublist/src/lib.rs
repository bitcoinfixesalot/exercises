#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list.len() == second_list.len() && is_sub(first_list, second_list) {
        return Comparison::Equal;
    }
    if first_list.len() < second_list.len() && is_sub(first_list, second_list) {
        return Comparison::Sublist;
    }
    if first_list.len() > second_list.len() && is_sub(second_list, second_list) {
        return Comparison::Superlist;
    }
    Comparison::Unequal
}

fn is_sub<T: PartialEq>(short: &[T], long: &[T]) -> bool {
    short.is_empty() || long.windows(short.len()).any(|l| l == short)
}
