use std::time::Instant;

use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

#[allow(clippy::ptr_arg, unused_variables)]
pub fn daxpy(a: &mut Vec<f64>, b: &Vec<f64>, scalar: f64, n: usize) -> f64 {
    let s = Instant::now();

    a.par_iter_mut()
        .enumerate()
        .for_each(|(i, x)| *x += scalar * b[i]);

    // Serial version
    // for i in 0..n {
    //     a[i] += scalar * b[i];
    // }

    s.elapsed().as_secs_f64()
}
