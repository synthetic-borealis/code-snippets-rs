pub fn gnome_sort<T>(v: &mut Vec<T>)
where
    T: Ord + Copy,
{
    let mut pos: usize = 0;
    while pos < v.len() {
        if pos == 0 || v[pos] >= v[pos - 1] {
            pos += 1;
        } else {
            v.swap(pos, pos - 1);
            pos -= 1;
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
        gnome_sort(&mut test_vector);
        assert_eq!(test_vector, expected);
    }
}
