use criterion::{criterion_group, criterion_main, BatchSize, Criterion, SamplingMode};
use rust_project_fiit_stu::{dll, immutable_ll, immutable_thread_safe_ll, ll};

criterion_group!(benches, bench_push, bench_pop);
criterion_main!(benches);

const N: usize = 100000;

fn bench_push(c: &mut Criterion) {
    let mut group = c.benchmark_group("Push");
    group.sampling_mode(SamplingMode::Flat);

    group.bench_function("Vec", |b| {
        b.iter_batched(
            || Vec::new(),
            |mut v| {
                for i in 0..N {
                    v.push(i);
                }
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("Std LL", |b| {
        b.iter_batched(
            || std::collections::LinkedList::new(),
            |mut std_ll| {
                for i in 0..N {
                    std_ll.push_front(i);
                }
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("LL", |b| {
        b.iter_batched(
            || ll::LinkedList::new(),
            |mut ll| {
                for i in 0..N {
                    ll.push(i);
                }
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("Immutable LL", |b| {
        b.iter_batched(
            || immutable_ll::ImmutableLinkedList::new(),
            |mut immutable_ll| {
                for i in 0..N {
                    immutable_ll = immutable_ll.prepend(i);
                }
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("Thread Safe LL", |b| {
        b.iter_batched(
            || immutable_thread_safe_ll::ImmutableLinkedList::new(),
            |mut immutable_thread_safe_ll| {
                for i in 0..N {
                    immutable_thread_safe_ll = immutable_thread_safe_ll.prepend(i);
                }
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("DLL", |b| {
        b.iter_batched(
            || dll::DoublyLinkedList::new(),
            |mut dll| {
                for i in 0..N {
                    dll.push_front(i);
                }
            },
            BatchSize::SmallInput,
        );
    });
}

fn bench_pop(c: &mut Criterion) {
    let mut group = c.benchmark_group("Pop");
    group.sampling_mode(SamplingMode::Flat);

    group.bench_function("Vec", |b| {
        b.iter_batched(
            || {
                let mut v = Vec::new();
                for i in 0..N {
                    v.push(i);
                }
                v
            },
            |mut v| {
                for _ in 0..N {
                    v.pop();
                }
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("Std LL", |b| {
        b.iter_batched(
            || {
                let mut std_ll = std::collections::LinkedList::new();
                for i in 0..N {
                    std_ll.push_front(i);
                }
                std_ll
            },
            |mut std_ll| {
                for _ in 0..N {
                    std_ll.pop_front();
                }
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("LL", |b| {
        b.iter_batched(
            || {
                let mut ll = ll::LinkedList::new();
                for i in 0..N {
                    ll.push(i);
                }
                ll
            },
            |mut ll| {
                for _ in 0..N {
                    ll.pop();
                }
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("Immutable LL", |b| {
        b.iter_batched(
            || {
                let mut immutable_ll = immutable_ll::ImmutableLinkedList::new();
                for i in 0..N {
                    immutable_ll = immutable_ll.prepend(i);
                }
                immutable_ll
            },
            |mut immutable_ll| {
                for _ in 0..N {
                    immutable_ll = immutable_ll.tail();
                }
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("Thread Safe LL", |b| {
        b.iter_batched(
            || {
                let mut immutable_thread_safe_ll =
                    immutable_thread_safe_ll::ImmutableLinkedList::new();
                for i in 0..N {
                    immutable_thread_safe_ll = immutable_thread_safe_ll.prepend(i);
                }
                immutable_thread_safe_ll
            },
            |mut immutable_thread_safe_ll| {
                for _ in 0..N {
                    immutable_thread_safe_ll = immutable_thread_safe_ll.tail();
                }
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("DLL", |b| {
        b.iter_batched(
            || {
                let mut dll = dll::DoublyLinkedList::new();
                for i in 0..N {
                    dll.push_front(i);
                }
                dll
            },
            |mut dll| {
                for _ in 0..N {
                    dll.pop_front();
                }
            },
            BatchSize::SmallInput,
        );
    });
}
