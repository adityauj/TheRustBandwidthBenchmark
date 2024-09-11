use std::time::Instant;

#[allow(clippy::ptr_arg)]
pub fn striad(
    a: &mut Vec<f64>,
    b: &mut Vec<f64>,
    c: &mut Vec<f64>,
    d: &mut Vec<f64>,
    n: usize,
) -> f64 {
    let s = Instant::now();

    for i in 0..n {
        a[i] = b[i] + d[i] * c[i];
    }

    s.elapsed().as_secs_f64()
}
