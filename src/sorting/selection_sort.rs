pub fn selection_sort<T>(v: &mut Vec<T>)
where
    T: Ord,
{
    for i in 0..v.len() - 1 {
        let mut min_idx: usize = i;

        for j in i..v.len() {
            if v[j] < v[min_idx] {
                min_idx = j;
            }
        }

        if min_idx != i {
            v.swap(i, min_idx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_small_array() {
        let mut test_vector = vec![5, 1, 8, -12, 22, 9];
        let expected = vec![-12, 1, 5, 8, 9, 22];
        selection_sort(&mut test_vector);
        assert_eq!(test_vector, expected);
    }
}
