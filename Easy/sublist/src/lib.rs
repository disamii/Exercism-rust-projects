#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    use Comparison::*;

    // Both empty
    if first_list.is_empty() && second_list.is_empty() {
        return Equal;
    }

    // Empty first list is always sublist
    if first_list.is_empty() {
        return Sublist;
    }

    // Empty second list is always superlist
    if second_list.is_empty() {
        return Superlist;
    }

    // Exact equality
    if first_list == second_list {
        return Equal;
    }

    // Check sublist
    if first_list.len() < second_list.len() {
        if second_list.windows(first_list.len()).any(|w| w == first_list) {
            return Sublist;
        }
    }

    // Check superlist
    if first_list.len() > second_list.len() {
        if first_list.windows(second_list.len()).any(|w| w == second_list) {
            return Superlist;
        }
    }

    Unequal
}
