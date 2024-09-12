use std::time::Instant;

use rayon::{iter::ParallelIterator, slice::ParallelSliceMut};

#[allow(clippy::ptr_arg, unused_variables)]
pub fn sdaxpy(a: &mut [f64], b: &[f64], c: &[f64], n: usize) -> f64 {
    let a_iter = a.par_chunks_mut(n);

    let s = Instant::now();

    // Serial version
    // for i in 0..n {
    //     a[i] += b[i] * c[i];
    // }

    // Parallel version
    a_iter.for_each(|a_slice| {
        a_slice
            .iter_mut()
            .enumerate()
            .for_each(|(i, val)| *val = c[i].mul_add(b[i], *val))
    });

    s.elapsed().as_secs_f64()
}
