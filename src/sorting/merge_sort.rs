use std::collections::VecDeque;

pub fn merge_sort<T>(v: &mut VecDeque<T>)
where
    T: Ord + Copy,
{
    if v.len() > 1 {
        let mid = v.len() / 2;
        let mut right = v.split_off(mid);
        let mut left = v.clone();
        merge_sort(&mut right);
        merge_sort(&mut left);
        *v = merge(left, right);
    }
}

fn merge<T>(mut left: VecDeque<T>, mut right: VecDeque<T>) -> VecDeque<T>
where
    T: Ord + Copy,
{
    let mut merged: VecDeque<T> = vec![].into();

    while left.front().is_some() && right.front().is_some() {
        let left_front = left.front().unwrap();
        let right_front = right.front().unwrap();

        match left_front.cmp(right_front) {
            std::cmp::Ordering::Less => {
                merged.push_back(left.pop_front().unwrap());
            }
            std::cmp::Ordering::Greater => {
                merged.push_back(right.pop_front().unwrap());
            }
            std::cmp::Ordering::Equal => {
                merged.push_back(left.pop_front().unwrap());
                merged.push_back(right.pop_front().unwrap());
            }
        }
    }
    while left.front().is_some() {
        merged.push_back(left.pop_front().unwrap());
    }
    while right.front().is_some() {
        merged.push_back(right.pop_front().unwrap());
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_small_array() {
        let mut test_vector = VecDeque::from([5, 1, 8, -12, 22, 9]);
        let expected = VecDeque::from([-12, 1, 5, 8, 9, 22]);
        merge_sort(&mut test_vector);
        assert_eq!(test_vector, expected);
    }
}
