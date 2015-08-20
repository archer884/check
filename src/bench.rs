use std::sync::Arc;
use test;

lazy_static! {
    static ref N: usize = 1;
}

// Based on these benchmarks, it looks to me as if `lazy_static` is definitively the faster of the
// two options I have (realistically) for making something accessible to multiple threads when
// dealing with lines of the length I'm working with.
//
//      running 2 tests
//      test bench::access_arc          ... bench:      36 ns/iter (+/- 41)
//      test bench::access_lazy_static  ... bench:       0 ns/iter (+/- 1)
//
// So I'm gonna keep doing it that way. :p

#[bench]
fn access_lazy_static(b: &mut test::Bencher) {
    b.iter(|| test::black_box(&N))
}

#[bench]
fn access_arc(b: &mut test::Bencher) {
    let arc = Arc::new(1usize);

    b.iter(|| test::black_box(arc.clone()))
}
