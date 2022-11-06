use crate::sorting::utils::is_sorted;
use rand::prelude::*;

const MAX_ITERATIONS: usize = 300000;

pub fn bozo_sort<T>(v: &mut Vec<T>)
where
    T: Ord,
{
    let mut iteration: usize = 0;
    let mut rng = thread_rng();
    while !is_sorted(v) && iteration < MAX_ITERATIONS {
        let i: usize = rng.gen_range(0..v.len());
        let mut j: usize = rng.gen_range(0..v.len());
        while i == j {
            j = rng.gen_range(0..v.len());
        }
        v.swap(i, j);
        iteration += 1;
    }
}
