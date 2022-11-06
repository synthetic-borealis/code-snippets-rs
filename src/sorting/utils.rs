pub fn is_sorted<T>(v: &Vec<T>) -> bool
where
    T: Ord,
{
    for i in 0..v.len() - 1 {
        if v[i] > v[i + 1] {
            return false;
        }
    }
    true
}
