pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }
    let middle_index = array.len() / 2;
    let middle = array[middle_index];
    if middle == key {
        return Some(middle_index);
    } else if middle < key {
        if let Some(idx) = find(&array[middle_index + 1..], key) {
            return Some(middle_index + 1 + idx);
        } else {
            return None;
        }
    } else {
        return find(&array[..middle_index], key);
    }

}
