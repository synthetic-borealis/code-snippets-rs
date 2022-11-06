pub fn bubble_sort<T>(v: &mut Vec<T>)
where
    T: Ord + Copy,
{
    loop {
        let mut swaps = 0;

        for i in 0..v.len() - 1 {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
                swaps += 1;
            }
        }

        if swaps == 0 {
            break;
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
        bubble_sort(&mut test_vector);
        assert_eq!(test_vector, expected);
    }
}
