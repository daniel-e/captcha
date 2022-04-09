extern crate captcha;
extern crate time;

use captcha::{gen, Difficulty};
use std::thread;
use time::Instant;

fn main() {
    let n = 500;
    let nthreads: i64 = 8;
    let mut threads = vec![];

    let b = Instant::now();

    for _ in 0..nthreads {
        let h = thread::spawn(move || {
            for _ in 0..n {
                gen(Difficulty::Easy).as_tuple();
            }
            println!("done {:?} ms", b.elapsed().whole_milliseconds());
        });
        threads.push(h);
    }

    for i in threads {
        i.join().expect("join failed");
    }

    let d = b.elapsed();
    println!("n                     : {}", n * nthreads);
    println!("time in ms total      : {}", d.whole_milliseconds());
    println!(
        "time in ms per captcha: {}",
        d.whole_milliseconds() as f64 / (n * nthreads) as f64
    );
    println!(
        "#captchs per second   : {}",
        (n * nthreads * 1000) / (d.whole_milliseconds() as i64)
    );
}
