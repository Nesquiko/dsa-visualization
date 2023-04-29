use criterion::{criterion_group, criterion_main, BatchSize, Criterion, SamplingMode};
use rust_project_fiit_stu::{dll, immutable_ll, immutable_thread_safe_ll, ll};

criterion_group!(big_data_ops, bench_push, bench_pop, bench_get);
criterion_main!(big_data_ops);

const N: usize = 100;
const M: usize = 10_000;

fn bench_push(c: &mut Criterion) {
    let mut group = c.benchmark_group("BigData-Push");
    group.sampling_mode(SamplingMode::Flat);

    let mut big_arr: [u64; M] = [0; M];
    for i in 0..M {
        big_arr[i] = i as u64;
    }

    group.bench_function("Vec", |b| {
        b.iter_batched(
            || Vec::new(),
            |mut v| {
                for _ in 0..N {
                    v.push(big_arr.clone());
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
                    std_ll.push_front(big_arr.clone());
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
                    ll.push(big_arr.clone());
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
                    immutable_ll = immutable_ll.prepend(big_arr.clone());
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
                    immutable_thread_safe_ll = immutable_thread_safe_ll.prepend(big_arr.clone());
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
                    dll.push_front(big_arr.clone());
                }
            },
            BatchSize::SmallInput,
        );
    });
}

fn bench_pop(c: &mut Criterion) {
    let mut group = c.benchmark_group("BigData-Pop");
    group.sampling_mode(SamplingMode::Flat);

    let mut big_arr: [u64; M] = [0; M];
    for i in 0..M {
        big_arr[i] = i as u64;
    }

    let mut blackhole: Option<[u64; M]> = None;
    group.bench_function("Vec", |b| {
        b.iter_batched(
            move || {
                let mut v = Vec::new();
                for _ in 0..N {
                    v.push(big_arr.clone());
                }
                v
            },
            |mut v| {
                for _ in 0..N {
                    blackhole = v.pop();
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
                    std_ll.push_front(big_arr.clone());
                }
                std_ll
            },
            |mut std_ll| {
                for _ in 0..N {
                    blackhole = std_ll.pop_front();
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
                    ll.push(big_arr.clone());
                }
                ll
            },
            |mut ll| {
                for _ in 0..N {
                    blackhole = ll.pop();
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
                    immutable_ll = immutable_ll.prepend(big_arr.clone());
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
                    immutable_thread_safe_ll = immutable_thread_safe_ll.prepend(big_arr.clone());
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
                    dll.push_front(big_arr.clone());
                }
                dll
            },
            |mut dll| {
                for _ in 0..N {
                    blackhole = dll.pop_front();
                }
            },
            BatchSize::SmallInput,
        );
    });
}

fn bench_get(c: &mut Criterion) {
    let mut group = c.benchmark_group("BigData-Get");
    group.sampling_mode(SamplingMode::Flat);

    let mut big_arr: [u64; M] = [0; M];
    for i in 0..M {
        big_arr[i] = i as u64;
    }

    group.bench_function("Vec", |b| {
        let mut v = Vec::new();
        for _ in 0..N {
            v.push(big_arr.clone());
        }

        b.iter(|| v.get(N / 2))
    });

    // STD LinkedList does not support indexing

    group.bench_function("LL", |b| {
        let mut ll = ll::LinkedList::new();
        for _ in 0..N {
            ll.push(big_arr.clone());
        }

        b.iter(|| ll.get(N / 2))
    });

    group.bench_function("Immutable LL", |b| {
        let mut immutable_ll = immutable_ll::ImmutableLinkedList::new();
        for _ in 0..N {
            immutable_ll = immutable_ll.prepend(big_arr.clone());
        }

        b.iter(|| immutable_ll.get(N / 2))
    });

    group.bench_function("Thread Safe LL", |b| {
        let mut immutable_thread_safe_ll = immutable_thread_safe_ll::ImmutableLinkedList::new();
        for _ in 0..N {
            immutable_thread_safe_ll = immutable_thread_safe_ll.prepend(big_arr.clone());
        }

        b.iter(|| immutable_thread_safe_ll.get(N / 2))
    });

    // DoublyLinkedList does not support indexing
}
