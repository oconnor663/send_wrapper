#![feature(test)]

extern crate test;

use send_wrapper::SendWrapper;
use test::Bencher;

#[bench]
fn bench_deref(b: &mut Bencher) {
	let x = SendWrapper::new(42);
	b.iter(|| *x);
}
