use std::time::Instant;

use rayon::{iter::ParallelIterator, slice::ParallelSliceMut};

#[allow(clippy::ptr_arg, unused_variables)]
pub fn striad(a: &mut [f64], b: &[f64], c: &[f64], d: &[f64], n: usize) -> f64 {
    let a_iter = a.par_chunks_mut(n);

    let s = Instant::now();

    // Serial version
    // for i in 0..n {
    //     a[i] = b[i] + d[i] * c[i];
    // }

    // Parallel version
    a_iter.for_each(|a_slice| {
        a_slice
            .iter_mut()
            .enumerate()
            .for_each(|(i, val)| *val = c[i].mul_add(d[i], b[i]))
    });
    s.elapsed().as_secs_f64()
}
