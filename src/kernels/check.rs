#[allow(unused_assignments, clippy::ptr_arg)]
pub fn check(a: &Vec<f64>, b: &Vec<f64>, c: &Vec<f64>, d: &Vec<f64>, n: usize, ntimes: usize) {
    /* reproduce initialization */
    let mut aj = 2.0;
    let mut bj = 2.0;
    let mut cj = 0.5;
    let mut dj = 1.0;

    let mut asum = 0.0;
    let mut bsum = 0.0;
    let mut csum = 0.0;
    let mut dsum = 0.0;
    let epsilon = 1.0e-8;

    /* now execute timing loop */
    let scalar = 3.0;

    for _ in 0..ntimes {
        bj = scalar;
        cj = aj;
        aj *= scalar;
        aj = bj + scalar * cj;
        aj += scalar * bj;
        aj = bj + cj * dj;
        aj += bj * cj;
    }

    aj *= n as f64;
    bj *= n as f64;
    cj *= n as f64;
    dj *= n as f64;

    for i in 0..n {
        asum += a[i];
        bsum += b[i];
        csum += c[i];
        dsum += d[i];
    }

    if f64::abs(aj - asum) / asum > epsilon {
        println!("Failed Validation on array a[]\n");
        println!("        Expected  : {} \n", aj);
        println!("        Observed  : {} \n", asum);
    } else if f64::abs(bj - bsum) / bsum > epsilon {
        println!("Failed Validation on array b[]\n");
        println!("        Expected  : {} \n", bj);
        println!("        Observed  : {} \n", bsum);
    } else if f64::abs(cj - csum) / csum > epsilon {
        println!("Failed Validation on array c[]\n");
        println!("        Expected  : {} \n", cj);
        println!("        Observed  : {} \n", csum);
    } else if f64::abs(dj - dsum) / dsum > epsilon {
        println!("Failed Validation on array d[]\n");
        println!("        Expected  : {} \n", dj);
        println!("        Observed  : {} \n", dsum);
    } else {
        println!("Solution Validates\n");
    }
}
