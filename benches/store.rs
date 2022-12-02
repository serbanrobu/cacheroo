#![feature(test)]

extern crate test;

use cacheroo::Store;
use test::Bencher;

fn store_insert_n(size: usize, b: &mut Bencher) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    b.iter(|| {
        let store = Store::new();
        let size = test::black_box(size);

        rt.block_on(async {
            for i in 0..size {
                store.insert(i, i).await;
            }
        });
    })
}

#[bench]
fn store_insert_10(b: &mut Bencher) {
    store_insert_n(10, b)
}

#[bench]
fn store_insert_100(b: &mut Bencher) {
    store_insert_n(100, b)
}

#[bench]
fn store_insert_1000(b: &mut Bencher) {
    store_insert_n(1000, b)
}

fn store_lookup_n(size: usize, b: &mut Bencher) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let store = Store::new();

    rt.block_on(async {
        for i in 0..size {
            store.insert(i, i).await;
        }
    });

    b.iter(|| {
        let size = test::black_box(size);

        rt.block_on(async {
            for i in 0..size {
                store.get(&i).await;
            }
        });
    })
}

#[bench]
fn store_lookup_10(b: &mut Bencher) {
    store_lookup_n(10, b)
}

#[bench]
fn store_lookup_100(b: &mut Bencher) {
    store_lookup_n(100, b)
}

#[bench]
fn store_lookup_1000(b: &mut Bencher) {
    store_lookup_n(1000, b)
}
