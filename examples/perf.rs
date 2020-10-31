extern crate captcha;
extern crate time;

use captcha::{gen, Difficulty};
use std::thread;
use time::PreciseTime;

fn main() {
    let n = 500;
    let nthreads = 8;
    let mut threads = vec![];

    let b = PreciseTime::now();

    for _ in 0..nthreads {
        let h = thread::spawn(move || {
            for _ in 0..n {
                gen(Difficulty::Easy).as_tuple();
            }
            println!("done {}ms", time::precise_time_ns() / 1000 / 1000);
        });
        threads.push(h);
    }

    for i in threads {
        i.join().expect("join failed");
    }

    let c = PreciseTime::now();
    let d = b.to(c).num_milliseconds();
    println!("n                     : {}", n * nthreads);
    println!("time in ms total      : {}", d);
    println!("time in ms per captcha: {}", d / (n * nthreads));
    println!("#captchs per second   : {}", n * nthreads * 1000 / d);
}
