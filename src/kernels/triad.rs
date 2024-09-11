use std::time::Instant;

#[allow(clippy::ptr_arg)]
pub fn triad(a: &mut Vec<f64>, b: &Vec<f64>, c: &Vec<f64>, scalar: f64, n: usize) -> f64 {
    let s = Instant::now();

    for i in 0..n {
        a[i] = b[i] + scalar * c[i];
    }

    s.elapsed().as_secs_f64()
}
