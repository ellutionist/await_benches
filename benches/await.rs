use criterion::{criterion_group, criterion_main, Criterion};
use noop_waker::noop_waker;
use std::future::Future;
use std::hint::black_box;
use std::pin::pin;

struct YieldNow;

impl std::future::Future for YieldNow {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        std::task::Poll::Pending
    }
}

#[inline]
async fn async_loop() {
    #[allow(unused_variables)]
    let mut i: i32 = 0;
    loop {
        i = black_box(i.wrapping_add(1));
        YieldNow.await;
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("await", |b| {
        let mut fut = pin!(async_loop());
        let waker = noop_waker();
        let mut ctx = std::task::Context::from_waker(&waker);

        b.iter(|| {
            let _ = black_box(fut.as_mut().poll(&mut ctx));
        })
    });
    c.bench_function("await5", |b| {
        let mut fut = pin!(await5());
        let waker = noop_waker();
        let mut ctx = std::task::Context::from_waker(&waker);

        b.iter(|| {
            let _ = black_box(fut.as_mut().poll(&mut ctx));
        })
    });

    c.bench_function("await10", |b| {
        let mut fut = pin!(await10());
        let waker = noop_waker();
        let mut ctx = std::task::Context::from_waker(&waker);

        b.iter(|| {
            let _ = black_box(fut.as_mut().poll(&mut ctx));
        })
    });

    c.bench_function("await20", |b| {
        let mut fut = pin!(await20());
        let waker = noop_waker();
        let mut ctx = std::task::Context::from_waker(&waker);

        b.iter(|| {
            let _ = black_box(fut.as_mut().poll(&mut ctx));
        })
    });

    c.bench_function("await30", |b| {
        let mut fut = pin!(await30());
        let waker = noop_waker();
        let mut ctx = std::task::Context::from_waker(&waker);

        b.iter(|| {
            let _ = black_box(fut.as_mut().poll(&mut ctx));
        })
    });

    c.bench_function("await40", |b| {
        let mut fut = pin!(await40());
        let waker = noop_waker();
        let mut ctx = std::task::Context::from_waker(&waker);

        b.iter(|| {
            let _ = black_box(fut.as_mut().poll(&mut ctx));
        })
    });

    c.bench_function("await50", |b| {
        let mut fut = pin!(await50());
        let waker = noop_waker();
        let mut ctx = std::task::Context::from_waker(&waker);

        b.iter(|| {
            let _ = black_box(fut.as_mut().poll(&mut ctx));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

#[inline]
async fn await1() {
    async_loop().await;
}

#[inline]
async fn await2() {
    await1().await;
}

#[inline]
async fn await3() {
    await2().await;
}

#[inline]
async fn await4() {
    await3().await;
}

#[inline]
async fn await5() {
    await4().await;
}

async fn await6() {
    await5().await;
}

async fn await7() {
    await6().await;
}

async fn await8() {
    await7().await;
}

async fn await9() {
    await8().await;
}

async fn await10() {
    await9().await;
}

// generate await11 to await100

async fn await11() {
    await10().await;
}

async fn await12() {
    await11().await;
}

async fn await13() {
    await12().await;
}

async fn await14() {
    await13().await;
}

async fn await15() {
    await14().await;
}

async fn await16() {
    await15().await;
}

async fn await17() {
    await16().await;
}

async fn await18() {
    await17().await;
}

async fn await19() {
    await18().await;
}

async fn await20() {
    await19().await;
}

async fn await21() {
    await20().await;
}

async fn await22() {
    await21().await;
}

async fn await23() {
    await22().await;
}

async fn await24() {
    await23().await;
}

async fn await25() {
    await24().await;
}

async fn await26() {
    await25().await;
}

async fn await27() {
    await26().await;
}

async fn await28() {
    await27().await;
}

async fn await29() {
    await28().await;
}

async fn await30() {
    await29().await;
}

async fn await31() {
    await30().await;
}

async fn await32() {
    await31().await;
}

async fn await33() {
    await32().await;
}

async fn await34() {
    await33().await;
}

async fn await35() {
    await34().await;
}

async fn await36() {
    await35().await;
}

async fn await37() {
    await36().await;
}

async fn await38() {
    await37().await;
}

async fn await39() {
    await38().await;
}

async fn await40() {
    await39().await;
}

async fn await41() {
    await40().await;
}

async fn await42() {
    await41().await;
}

async fn await43() {
    await42().await;
}

async fn await44() {
    await43().await;
}

async fn await45() {
    await44().await;
}

async fn await46() {
    await45().await;
}

async fn await47() {
    await46().await;
}

async fn await48() {
    await47().await;
}

async fn await49() {
    await48().await;
}

async fn await50() {
    await49().await;
}
