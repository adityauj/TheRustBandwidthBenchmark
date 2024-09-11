use std::time::Instant;

#[allow(clippy::ptr_arg)]
pub fn sum(a: &mut Vec<f64>, n: usize) -> f64 {
    let s = Instant::now();

    let mut sum = 0.0;
    for i in a.iter().take(n) {
        sum += *i;
    }

    let e = s.elapsed();

    a[10] = sum;

    e.as_secs_f64()
}
