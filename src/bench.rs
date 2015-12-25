use std::sync::Arc;
use test;

// Based on these benchmarks, it looks to me as if `lazy_static` is
// definitively the faster of the
// two options I have (realistically) for making something accessible to
// multiple threads when
// dealing with lines of the length I'm working with.
//
//      running 2 tests
//      test bench::access_arc          ... bench:      36 ns/iter (+/- 41)
//      test bench::access_lazy_static  ... bench:       0 ns/iter (+/- 1)
//
// Update: Further testing demonstrates that accessing an arc one hundred times
// has a cost of
// almost nothing, while accessing a lazy_static 100 times has a cost of around
// 113 ns (on my
// gaming rig, anyway... the original tests were run on a different, slower
// machine). This means,
// basically, that the tipping point is much lower than I had previously
// thought. (The difference
// between cloning and accessing on this machine is also much smaller--although
// I think that just
// means that the value of '0' for reading is smaller on this machine than it
// was on the other.)
//
// In short, the arc is almost certainly going to be faster for most workloads,
// particularly now
// that the issue with sending empty workpieces to the threadpool has been
// solved.

lazy_static! {
    static ref N: usize = 1;
}

#[bench]
fn access_lazy_static(b: &mut test::Bencher) {
    b.iter(|| test::black_box(&N))
}

#[bench]
fn access_arc(b: &mut test::Bencher) {
    let arc = Arc::new(1usize);

    b.iter(|| test::black_box(arc.clone()))
}

#[bench]
fn multi_access_lazy_static(b: &mut test::Bencher) {
    b.iter(|| {
        let mut total = 0;
        for _ in 0..100 {
            total += *N;
        }
        test::black_box(total);
    })
}

#[bench]
fn multi_access_arc(b: &mut test::Bencher) {
    let arc = Arc::new(1usize);

    b.iter(|| {
        let mut total = 0;
        for _ in 0..100 {
            total += *arc;
        }
        test::black_box(total);
    })
}
