use criterion::{criterion_group, criterion_main, black_box, Criterion};

use expr_bench::add_two;

pub fn add_two_const_bench(c: &mut Criterion) {
    c.bench_function("add_two", |b| {
        b.iter(|| {
            add_two(black_box(2));
        });
    });
}

pub fn add_two_const_twice_bench(c: &mut Criterion) {
    c.bench_function("add_two_twice", |b| {
        b.iter(|| {
	    // I'd expect this to be twice as long as add_two_const_bench
            add_two(black_box(2));
            add_two(black_box(2));
        });
    });
}

pub fn add_two_stack_bench(c: &mut Criterion) {
    c.bench_function("add_two_stack", |b| {
	let stack_var = 2;
        b.iter(|| {
            add_two(black_box(stack_var));
        });
    });
}

criterion_group!(benches, add_two_const_bench, add_two_const_twice_bench, add_two_stack_bench);
criterion_main!(benches);
