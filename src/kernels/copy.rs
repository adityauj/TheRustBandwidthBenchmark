use std::time::Instant;

#[allow(clippy::ptr_arg, clippy::manual_memcpy)]
pub fn copy(c: &mut Vec<f64>, a: &Vec<f64>, n: usize) -> f64 {
    let s = Instant::now();

    for i in 0..n {
        c[i] = a[i];
    }

    s.elapsed().as_secs_f64()
}
