use std::time::Instant;

use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    slice::{ParallelSlice, ParallelSliceMut},
};

#[allow(clippy::ptr_arg, unused_variables)]
#[inline(never)]
pub fn triad(a: &mut [f64], b: &[f64], c: &[f64], scalar: f64, n: usize, block_size: usize) -> f64 {
    let a = &mut a[..n];
    let b = &b[..n];
    let c = &c[..n];

    let a_iter = a.par_chunks_mut(block_size);
    let b_iter = b.par_chunks(block_size);
    let c_iter = c.par_chunks(block_size);

    let s = Instant::now();

    // // Serial version
    // for i in (0..n) {
    //     a[i] = c[i] * scalar + b[i];
    // }

    // Parallel version
    a_iter
        .zip((b_iter, c_iter))
        .for_each(|(a_slice, (b_slice, c_slice))| {
            a_slice
                .iter_mut()
                .enumerate()
                .for_each(|(i, val)| *val = c_slice[i] * scalar + b_slice[i])
        });

    s.elapsed().as_secs_f64()
}
