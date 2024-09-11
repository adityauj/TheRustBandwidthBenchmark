use std::time::Instant;

use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

#[allow(clippy::ptr_arg, unused_variables)]
pub fn update(b: &mut Vec<f64>, scalar: f64, n: usize) -> f64 {
    let s = Instant::now();

    b.par_iter_mut().for_each(|x| *x += scalar);

    // Serial version
    // for i in b.iter_mut().take(n) {
    //     *i += scalar;
    // }

    s.elapsed().as_secs_f64()
}
