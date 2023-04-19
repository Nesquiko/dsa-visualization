use criterion::{criterion_group, criterion_main, Criterion};
use rust_project_fiit_stu::{dll, immutable_ll, immutable_thread_safe_ll, ll};

fn bench_vec_vs_ll(c: &mut Criterion) {
    c.bench_function("Vec", |b| b.iter(|| vec()));
    c.bench_function("LL", |b| b.iter(|| ll()));
    c.bench_function("Immutable LL", |b| b.iter(|| immutable_ll()));
    c.bench_function("Immutable Thread Safe LL", |b| {
        b.iter(|| immutable_thread_safe_ll())
    });
    c.bench_function("DLL", |b| b.iter(|| dll()));
}

criterion_group!(benches, bench_vec_vs_ll);
criterion_main!(benches);

fn vec() {
    let mut vec = Vec::new();
    for i in 0..1000000 {
        vec.push(i);
    }
}

fn ll() {
    let mut ll = ll::LinkedList::new();
    for i in 0..1000000 {
        ll.push(i);
    }
}

fn immutable_ll() {
    let mut immutable_ll = immutable_ll::ImmutableLinkedList::new();
    for i in 0..1000000 {
        immutable_ll = immutable_ll.prepend(i);
    }
}

fn immutable_thread_safe_ll() {
    let mut immutable_thread_safe_ll = immutable_thread_safe_ll::ImmutableLinkedList::new();
    for i in 0..1000000 {
        immutable_thread_safe_ll = immutable_thread_safe_ll.prepend(i);
    }
}

fn dll() {
    let mut dll = dll::DoublyLinkedList::new();
    for i in 0..1000000 {
        dll.push_front(i);
    }
}
