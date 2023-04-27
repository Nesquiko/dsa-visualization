use criterion::{criterion_group, criterion_main, BatchSize, Criterion, SamplingMode};
use rust_project_fiit_stu::{dll, immutable_ll, immutable_thread_safe_ll, ll};

criterion_group!(big_data_ops, bench_push, bench_pop, bench_get);
criterion_main!(big_data_ops);

const N: usize = 1000;
const M: usize = 10_000;

fn bench_push(c: &mut Criterion) {
    let mut group = c.benchmark_group("BigData-Push");
    let big_string = "a".repeat(M);
    group.sampling_mode(SamplingMode::Flat);

    group.bench_function("Vec", |b| {
        b.iter_batched(
            || Vec::new(),
            |mut v| {
                for _ in 0..N {
                    v.push(big_string.clone());
                }
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("Std LL", |b| {
        b.iter_batched(
            || std::collections::LinkedList::new(),
            |mut std_ll| {
                for _ in 0..N {
                    std_ll.push_front(big_string.clone());
                }
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("LL", |b| {
        b.iter_batched(
            || ll::LinkedList::new(),
            |mut ll| {
                for _ in 0..N {
                    ll.push(big_string.clone());
                }
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("Immutable LL", |b| {
        b.iter_batched(
            || immutable_ll::ImmutableLinkedList::new(),
            |mut immutable_ll| {
                for _ in 0..N {
                    immutable_ll = immutable_ll.prepend(big_string.clone());
                }
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("Thread Safe LL", |b| {
        b.iter_batched(
            || immutable_thread_safe_ll::ImmutableLinkedList::new(),
            |mut immutable_thread_safe_ll| {
                for _ in 0..N {
                    immutable_thread_safe_ll = immutable_thread_safe_ll.prepend(big_string.clone());
                }
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("DLL", |b| {
        b.iter_batched(
            || dll::DoublyLinkedList::new(),
            |mut dll| {
                for _ in 0..N {
                    dll.push_front(big_string.clone());
                }
            },
            BatchSize::SmallInput,
        );
    });
}

fn bench_pop(c: &mut Criterion) {
    let mut group = c.benchmark_group("BigData-Pop");
    let big_string = "a".repeat(M);
    group.sampling_mode(SamplingMode::Flat);

    group.bench_function("Vec", |b| {
        b.iter_batched(
            || {
                let mut v = Vec::new();
                for _ in 0..N {
                    v.push(big_string.clone());
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
                for _ in 0..N {
                    std_ll.push_front(big_string.clone());
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
                for _ in 0..N {
                    ll.push(big_string.clone());
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
                for _ in 0..N {
                    immutable_ll = immutable_ll.prepend(big_string.clone());
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
                for _ in 0..N {
                    immutable_thread_safe_ll = immutable_thread_safe_ll.prepend(big_string.clone());
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
                for _ in 0..N {
                    dll.push_front(big_string.clone());
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

fn bench_get(c: &mut Criterion) {
    let mut group = c.benchmark_group("BigData-Get");
    let big_string = "a".repeat(M);
    group.sampling_mode(SamplingMode::Flat);

    group.bench_function("Vec", |b| {
        let mut v = Vec::new();
        for _ in 0..N {
            v.push(big_string.clone());
        }

        b.iter(|| v.get(N / 2))
    });

    // STD LinkedList does not support indexing

    group.bench_function("LL", |b| {
        let mut ll = ll::LinkedList::new();
        for _ in 0..N {
            ll.push(big_string.clone());
        }

        b.iter(|| ll.get(N / 2))
    });

    group.bench_function("Immutable LL", |b| {
        let mut immutable_ll = immutable_ll::ImmutableLinkedList::new();
        for _ in 0..N {
            immutable_ll = immutable_ll.prepend(big_string.clone());
        }

        b.iter(|| immutable_ll.get(N / 2))
    });

    group.bench_function("Thread Safe LL", |b| {
        let mut immutable_thread_safe_ll = immutable_thread_safe_ll::ImmutableLinkedList::new();
        for _ in 0..N {
            immutable_thread_safe_ll = immutable_thread_safe_ll.prepend(big_string.clone());
        }

        b.iter(|| immutable_thread_safe_ll.get(N / 2))
    });

    // DoublyLinkedList does not support indexing
}
