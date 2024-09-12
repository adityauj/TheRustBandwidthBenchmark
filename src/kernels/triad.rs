use std::time::Instant;

use rayon::{iter::ParallelIterator, slice::ParallelSliceMut};

#[allow(clippy::ptr_arg, unused_variables)]
#[inline(never)]
pub fn triad(a: &mut [f64], b: &[f64], c: &[f64], scalar: f64, n: usize) -> f64 {
    let a_iter = a.par_chunks_mut(n);

    let s = Instant::now();

    // Serial version
    // for i in 0..(n * 8) {
    //     a[i] = c[i].mul_add(scalar, b[i]);
    // }

    // Parallel version
    a_iter.for_each(|a_slice| {
        a_slice
            .iter_mut()
            .enumerate()
            .for_each(|(i, val)| *val = c[i].mul_add(scalar, b[i]))
    });

    s.elapsed().as_secs_f64()
}
