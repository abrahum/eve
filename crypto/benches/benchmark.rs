use bytes::BytesMut;
use criterion::{criterion_group, criterion_main, Criterion};
use crypto::tea::Tea;
use rand::{Rng, RngCore};

fn gen_tea() -> Tea {
    let mut rng = rand::thread_rng();
    let mut key = [0u32; 4];
    for i in 0..key.len() {
        key[i] = rng.gen();
    }
    Tea::new_with_u32(key)
}

fn encrypt() {
    let mut data = [0u8; 16];
    let tea = gen_tea();
    rand::thread_rng().fill_bytes(&mut data);
    let data = BytesMut::from(&data[..]).freeze();
    tea.encrypt(data);
}

fn enandde() {
    let mut data = [0u8; 16];
    let tea = gen_tea();
    rand::thread_rng().fill_bytes(&mut data);
    let data = BytesMut::from(&data[..]).freeze();
    let en = tea.encrypt(data.clone());
    assert_eq!(tea.decrypt(en).unwrap(), data);
}

fn bench(c: &mut Criterion) {
    c.bench_function("encrypt", |b| b.iter(|| encrypt()));
    c.bench_function("encrypt and decrypt", |b| b.iter(|| enandde()));
}

criterion_group!(benches, bench);
criterion_main!(benches);
