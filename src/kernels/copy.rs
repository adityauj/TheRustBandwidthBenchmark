use std::time::Instant;

use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

#[allow(clippy::ptr_arg, clippy::manual_memcpy, unused_variables)]
pub fn copy(c: &mut Vec<f64>, a: &Vec<f64>, n: usize) -> f64 {
    let s = Instant::now();

    c.par_iter_mut().enumerate().for_each(|(i, x)| *x = a[i]);

    // Serial version
    // for i in 0..n {
    //     c[i] = a[i];
    // }

    s.elapsed().as_secs_f64()
}
