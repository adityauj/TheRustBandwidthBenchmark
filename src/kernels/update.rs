use std::time::Instant;

use rayon::{iter::ParallelIterator, slice::ParallelSliceMut};

#[allow(clippy::ptr_arg, unused_variables)]
pub fn update(b: &mut [f64], scalar: f64, n: usize, block_size: usize) -> f64 {
    let b = &mut b[..n];

    let b_iter = b.par_chunks_mut(block_size);

    let s = Instant::now();

    // Serial version
    // for i in b.iter_mut().take(n) {
    //     *i += scalar;
    // }

    // Parallel version
    b_iter.for_each(|b_slice| b_slice.iter_mut().for_each(|val| *val += scalar));

    s.elapsed().as_secs_f64()
}
