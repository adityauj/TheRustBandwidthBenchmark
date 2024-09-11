mod kernels;
mod utils;

use clap::Parser;
use kernels::copy::copy;
use kernels::daxpy::daxpy;
use kernels::init::init;
use kernels::sdaxpy::sdaxpy;
use kernels::striad::striad;
use kernels::sum::sum;
use kernels::triad::triad;
use kernels::update::update;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::mem::size_of;
use std::time::Instant;

use crate::utils::arg_parser::ArgParser;
use crate::utils::benchmark::{Benchmark, BenchmarkType};

const HLINE: &str =
    "----------------------------------------------------------------------------\n";

macro_rules! bench {
    ($tag:expr, $func:expr, $times:expr, $index:expr) => {
        $times[$tag][$index] = $func;
    };
}

fn main() {
    let arg_parser = ArgParser::parse();

    rayon::ThreadPoolBuilder::new()
        .num_threads(arg_parser.n)
        .build_global()
        .unwrap();

    println!("Benchmarking with {:#?} workers.", arg_parser.n);

    const BYTES_PER_WORD: usize = size_of::<f64>();
    let n: usize = arg_parser.size;

    let num_of_benchmarks = Benchmark::Numbench as usize;

    let mut avgtime = vec![0.0; num_of_benchmarks];
    let mut maxtime = vec![0.0; num_of_benchmarks];
    let mut mintime = vec![f64::MAX; num_of_benchmarks];
    let mut times = vec![vec![0.0; arg_parser.ntimes]; num_of_benchmarks];

    let benchmarks = vec![
        BenchmarkType {
            label: "Init:   ".to_string(),
            words: 1,
            flops: 0,
        },
        BenchmarkType {
            label: "Sum:    ".to_string(),
            words: 1,
            flops: 0,
        },
        BenchmarkType {
            label: "Copy:   ".to_string(),
            words: 1,
            flops: 0,
        },
        BenchmarkType {
            label: "Update: ".to_string(),
            words: 1,
            flops: 0,
        },
        BenchmarkType {
            label: "Triad:  ".to_string(),
            words: 1,
            flops: 0,
        },
        BenchmarkType {
            label: "Daxpy:  ".to_string(),
            words: 1,
            flops: 0,
        },
        BenchmarkType {
            label: "STriad: ".to_string(),
            words: 1,
            flops: 0,
        },
        BenchmarkType {
            label: "SDaxpy: ".to_string(),
            words: 1,
            flops: 0,
        },
    ];

    let s = Instant::now();

    // Can also randomise the initialisation of arrays with rand crate : https://docs.rs/rand/0.8.5/rand/
    // let mut x: Arc<Vec<f64>> = Arc::new((0..n).into_par_iter().map(|_| (rand::random::<i32>() % 100) as f64 + 1.1).collect());

    let mut a: Vec<f64> = (0..n).into_par_iter().map(|_| 2.0).collect();
    let mut b: Vec<f64> = (0..n).into_par_iter().map(|_| 2.0).collect();
    let mut c: Vec<f64> = (0..n).into_par_iter().map(|_| 0.5).collect();
    let mut d: Vec<f64> = (0..n).into_par_iter().map(|_| 1.0).collect();

    let e = s.elapsed();

    println!("Initialization of arrays took : {e:#?}");

    let scalar = 3.0;

    for k in 0..arg_parser.ntimes {
        bench!(
            Benchmark::Init as usize,
            init(b.as_mut(), scalar, n),
            times,
            k
        );

        let tmp = a[10];

        bench!(Benchmark::Sum as usize, sum(a.as_mut(), n), times, k);

        a[10] = tmp;

        bench!(
            Benchmark::Copy as usize,
            copy(c.as_mut(), a.as_ref(), n),
            times,
            k
        );
        bench!(
            Benchmark::Update as usize,
            update(a.as_mut(), scalar, n),
            times,
            k
        );
        bench!(
            Benchmark::Triad as usize,
            triad(a.as_mut(), b.as_ref(), c.as_ref(), scalar, n),
            times,
            k
        );
        bench!(
            Benchmark::Daxpy as usize,
            daxpy(a.as_mut(), b.as_mut(), scalar, n),
            times,
            k
        );
        bench!(
            Benchmark::Striad as usize,
            striad(a.as_mut(), b.as_mut(), c.as_mut(), d.as_mut(), n),
            times,
            k
        );
        bench!(
            Benchmark::Sdaxpy as usize,
            sdaxpy(a.as_mut(), b.as_mut(), c.as_mut(), n),
            times,
            k
        );

        for j in 0..num_of_benchmarks {
            for k in 0..arg_parser.ntimes {
                avgtime[j] += times[j][k];
                mintime[j] = f64::min(mintime[j], times[j][k]);
                maxtime[j] = f64::max(maxtime[j], times[j][k]);
            }
        }
    }

    println!("{HLINE:#?}");
    println!("Function      Rate(MB/s)  Rate(MFlop/s)  Avg time     Min time     Max time\n");





    
}
