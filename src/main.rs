use criterion::{black_box, criterion_group, criterion_main, Criterion};

trait SlotSourceTrait {
    fn get_slot(&self) -> u64;
}

struct SlotSubscriber;

impl SlotSourceTrait for SlotSubscriber {
    fn get_slot(&self) -> u64 {
        1
    }
}

struct OrderSubscriber;

impl SlotSourceTrait for OrderSubscriber {
    fn get_slot(&self) -> u64 {
        2
    }
}

struct FooSubscriberTrait<T: SlotSourceTrait> {
    slot_source: T,
}

impl<T: SlotSourceTrait> FooSubscriberTrait<T> {
    pub fn new(slot_source: T) -> Self {
        Self { slot_source }
    }

    pub fn some_function(&self) {
        let slot = self.slot_source.get_slot();
        black_box(slot); // Use black_box to prevent optimizations
    }
}

fn benchmark_traits(c: &mut Criterion) {
    let slot_subscriber = SlotSubscriber;
    let order_subscriber = OrderSubscriber;

    let foo1 = FooSubscriberTrait::new(slot_subscriber);
    let foo2 = FooSubscriberTrait::new(order_subscriber);

    c.bench_function("traits_slot_subscriber", |b| {
        b.iter(|| foo1.some_function())
    });

    c.bench_function("traits_order_subscriber", |b| {
        b.iter(|| foo2.some_function())
    });
}

enum SlotSource {
    SlotSubscriber,
    OrderSubscriber,
}

impl SlotSource {
    fn get_slot(&self) -> u64 {
        match self {
            SlotSource::SlotSubscriber => 1,
            SlotSource::OrderSubscriber => 2,
        }
    }
}

struct FooSubscriber {
    slot_source: SlotSource,
}

impl FooSubscriber {
    pub fn new(slot_source: SlotSource) -> Self {
        Self { slot_source }
    }

    pub fn some_function(&self) {
        let slot = self.slot_source.get_slot();
        black_box(slot); // Use black_box to prevent optimizations
    }
}

fn benchmark_enums(c: &mut Criterion) {
    let slot_subscriber = SlotSource::SlotSubscriber;
    let order_subscriber = SlotSource::OrderSubscriber;

    let foo1 = FooSubscriber::new(slot_subscriber);
    let foo2 = FooSubscriber::new(order_subscriber);

    c.bench_function("enums_slot_subscriber", |b| b.iter(|| foo1.some_function()));

    c.bench_function("enums_order_subscriber", |b| {
        b.iter(|| foo2.some_function())
    });
}

criterion_group!(benches, benchmark_traits, benchmark_enums);
criterion_main!(benches);

//
// criterion_group!(benches, benchmark_enums);
// criterion_main!(benches);
