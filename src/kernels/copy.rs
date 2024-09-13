use std::time::Instant;

use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    slice::{ParallelSlice, ParallelSliceMut},
};

#[allow(clippy::ptr_arg, clippy::manual_memcpy, unused_variables)]
#[inline(never)]
pub fn copy(c: &mut [f64], a: &[f64], n: usize) -> f64 {
    let c_iter = c.par_chunks_mut(n);
    let a_iter = a.par_chunks(n);

    let s = Instant::now();

    // Serial version
    // for i in 0..n {
    //     c[i] = a[i];
    // }

    // Parallel version
    c_iter.zip(a_iter).for_each(|(c_slice, a_slice)| {
        c_slice.copy_from_slice(a_slice);
    });

    s.elapsed().as_secs_f64()
}
