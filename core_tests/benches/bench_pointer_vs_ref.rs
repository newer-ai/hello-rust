use criterion::{Criterion, criterion_group, criterion_main};

fn sum_safe(slice: &[i32]) -> i32 {
    let mut sum = 0;
    for x in slice.iter() {
        sum += *x;
    }
    sum
}

fn sum_unsafe(slice: &[i32]) -> i32 {
    let mut sum = 0;
    let ptr = slice.as_ptr();
    let len = slice.len();
    unsafe {
        for i in 0..len {
            sum += *ptr.add(i);
        }
    }
    sum
}

fn bench_pointer_vs_ref(c: &mut Criterion) {
    let v: Vec<_> = (0..100).collect();

    c.bench_function("safe_ref", |b| b.iter(|| sum_safe(&v)));
    c.bench_function("unsafe_ref", |b| b.iter(|| sum_unsafe(&v)));
}
criterion_group!(benches, bench_pointer_vs_ref);
criterion_main!(benches);
