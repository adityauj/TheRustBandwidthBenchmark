//=======================================================================================
//
//     Author:   Aditya Ujeniya (au), aditya.ujeniya@fau.de
//     Copyright (c) 2024 RRZE, University Erlangen-Nuremberg
//
//     Permission is hereby granted, free of charge, to any person obtaining a copy
//     of this software and associated documentation files (the "Software"), to deal
//     in the Software without restriction, including without limitation the rights
//     to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//     copies of the Software, and to permit persons to whom the Software is
//     furnished to do so, subject to the following conditions:
//
//     The above copyright notice and this permission notice shall be included in all
//     copies or substantial portions of the Software.
//
//     THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//     IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//     FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//     AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//     LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//     OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//     SOFTWARE.
//
//=======================================================================================

mod kernels;
mod utils;

use clap::Parser;
use kernels::check::check;
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
    "----------------------------------------------------------------------------------------------------------";

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

    println!("Benchmarking with {:#?} threads.", arg_parser.n);

    const BYTES_PER_WORD: usize = size_of::<f64>();
    let size = arg_parser.size;
    let ntimes = arg_parser.ntimes;
    let n_chunks = size / arg_parser.n;

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
            flops: 1,
        },
        BenchmarkType {
            label: "Copy:   ".to_string(),
            words: 2,
            flops: 0,
        },
        BenchmarkType {
            label: "Update: ".to_string(),
            words: 2,
            flops: 1,
        },
        BenchmarkType {
            label: "Triad:  ".to_string(),
            words: 3,
            flops: 2,
        },
        BenchmarkType {
            label: "Daxpy:  ".to_string(),
            words: 3,
            flops: 2,
        },
        BenchmarkType {
            label: "STriad: ".to_string(),
            words: 4,
            flops: 2,
        },
        BenchmarkType {
            label: "SDaxpy: ".to_string(),
            words: 4,
            flops: 2,
        },
    ];

    let s = Instant::now();

    // Can also randomise the initialisation of arrays with rand crate : https://docs.rs/rand/0.8.5/rand/
    // let mut x: Arc<Vec<f64>> = Arc::new((0..n).into_par_iter().map(|_| (rand::random::<i32>() % 100) as f64 + 1.1).collect());
    // But randomising will fail the check function at the end.

    let mut a: Vec<f64> = (0..size).into_par_iter().map(|_| 2.0).collect();
    let mut b: Vec<f64> = (0..size).into_par_iter().map(|_| 2.0).collect();
    let mut c: Vec<f64> = (0..size).into_par_iter().map(|_| 0.5).collect();
    let d: Vec<f64> = (0..size).into_par_iter().map(|_| 1.0).collect();

    let e = s.elapsed();
    println!(
        "Total allocated datasize: {:.2} MB.",
        4.0 * (BYTES_PER_WORD * size) as f64 * 1.0e-6
    );

    println!("Initialization of arrays took : {e:#?}.");

    let scalar = 3.0;

    for k in 0..ntimes {
        bench!(
            Benchmark::Init as usize,
            init(b.as_mut(), scalar, size, n_chunks),
            times,
            k
        );

        let tmp = a[10];

        bench!(
            Benchmark::Sum as usize,
            sum(a.as_mut(), size, n_chunks),
            times,
            k
        );

        a[10] = tmp;

        bench!(
            Benchmark::Copy as usize,
            copy(c.as_mut(), a.as_ref(), size, n_chunks),
            times,
            k
        );
        bench!(
            Benchmark::Update as usize,
            update(a.as_mut(), scalar, size, n_chunks),
            times,
            k
        );
        bench!(
            Benchmark::Triad as usize,
            triad(a.as_mut(), b.as_ref(), c.as_ref(), scalar, size, n_chunks),
            times,
            k
        );
        bench!(
            Benchmark::Daxpy as usize,
            daxpy(a.as_mut(), b.as_ref(), scalar, size, n_chunks),
            times,
            k
        );
        bench!(
            Benchmark::Striad as usize,
            striad(
                a.as_mut(),
                b.as_ref(),
                c.as_ref(),
                d.as_ref(),
                size,
                n_chunks
            ),
            times,
            k
        );
        bench!(
            Benchmark::Sdaxpy as usize,
            sdaxpy(a.as_mut(), b.as_ref(), c.as_ref(), size, n_chunks),
            times,
            k
        );
    }

    for j in 0..num_of_benchmarks {
        for k in 0..ntimes {
            avgtime[j] += times[j][k];
            mintime[j] = f64::min(mintime[j], times[j][k]);
            maxtime[j] = f64::max(maxtime[j], times[j][k]);
        }
    }
    println!("{HLINE}");
    println!(
        "{0: <15} | {1: <15} | {2: <15} | {3: <15}| {4: <15} | {5: <15} |",
        "Function", "Rate(MB/s)", "Rate(MFlop/s)", "Avg time", "Min time", "Max time"
    );
    println!("{HLINE}");

    for j in 0..num_of_benchmarks {
        avgtime[j] /= ntimes as f64;
        let bytes = benchmarks[j].words * BYTES_PER_WORD * size;
        let flops = benchmarks[j].flops * size;

        if flops > 0 {
            println!(
                "{0: <15} | {1: <15.2} | {2: <15.2} | {3: <15.4}| {4: <15.4} | {5: <15.4} |",
                benchmarks[j].label,
                1.0e-6 * bytes as f64 / mintime[j],
                1.0e-6 * flops as f64 / mintime[j],
                avgtime[j],
                mintime[j],
                maxtime[j]
            );
        } else {
            println!(
                "{0: <15} | {1: <15.2} | {2: <15} | {3: <15.4}| {4: <15.4} | {5: <15.4} |",
                benchmarks[j].label,
                1.0e-6 * bytes as f64 / mintime[j],
                "-",
                avgtime[j],
                mintime[j],
                maxtime[j]
            );
        }
    }
    println!("{HLINE}");

    check(a.as_ref(), b.as_ref(), c.as_ref(), d.as_ref(), size, ntimes);
}
