use bytes::BytesMut;
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

fn main() {
    // let mut data = [0u8; 16];
    // let tea = gen_tea();
    // rand::thread_rng().fill_bytes(&mut data);
    // let data = BytesMut::from(&data[..]).freeze();
    // let start = chrono::Local::now().timestamp_nanos();
    // for _ in 0..10000 {
    //     tea.encrypt(data.clone());
    // }
    // let end = chrono::Local::now().timestamp_nanos();
    // println!("{}", (end - start) / 10000);

    let mut data = [0u8; 16];
    let tea = gen_tea();
    static TIMES: usize = 100_000;
    rand::thread_rng().fill_bytes(&mut data);
    let data = BytesMut::from(&data[..]).freeze();
    let start = chrono::Local::now().timestamp_nanos();
    let en = tea.encrypt(data);
    for _ in 0..TIMES {
        tea.decrypt(en.clone()).unwrap();
    }
    let end = chrono::Local::now().timestamp_nanos();
    println!("{}", (end - start) as usize / TIMES);
}
