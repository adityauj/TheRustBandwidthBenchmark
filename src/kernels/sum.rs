use std::time::Instant;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[allow(clippy::ptr_arg, unused_variables)]
pub fn sum(a: &mut Vec<f64>, n: usize) -> f64 {
    let s = Instant::now();

    let sum = a.par_iter().sum();

    // Serial version
    // let mut sum = 0.0;
    // for i in a.iter().take(n) {
    //     sum += *i;
    // }

    let e = s.elapsed();

    a[10] = sum;

    e.as_secs_f64()
}
