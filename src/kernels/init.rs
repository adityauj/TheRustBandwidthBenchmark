use std::time::Instant;

#[allow(clippy::ptr_arg)]
pub fn init(b: &mut Vec<f64>, scalar: f64, n: usize) -> f64 {
    let s = Instant::now();

    for i in b.iter_mut().take(n) {
        *i = scalar;
    }

    s.elapsed().as_secs_f64()
}
