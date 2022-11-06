pub fn insertion_sort<T>(v: &mut Vec<T>)
where
    T: Ord + Copy,
{
    let mut i: usize = 1;
    let mut j: usize;

    while i < v.len() {
        j = i;

        while j > 0 && v[j - 1] > v[j] {
            v.swap(j - 1, j);
            j -= 1;
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_small_array() {
        let mut test_vector = vec![5, 1, 8, -12, 22, 9];
        let expected = vec![-12, 1, 5, 8, 9, 22];
        insertion_sort(&mut test_vector);
        assert_eq!(test_vector, expected);
    }
}
