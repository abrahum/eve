use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

mod client_protocol;
mod device_info;
mod version_info;

pub use client_protocol::ClientProtocol;
pub use device_info::DeviceInfo;
pub use version_info::VersionInfo;

pub fn gen_rand_number(len: usize) -> u64 {
    let mut n: u64 = 0;
    let mut rng = rand::thread_rng();
    for _ in 0..len {
        n = n * 10 + rng.gen_range(0..=9);
    }
    n
}

#[test]
fn gen_rand_number_test() {
    println!("{}", gen_rand_number(10));
}

pub fn gen_rand_string(len: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

#[test]
fn gen_rand_string_test() {
    println!("{}", gen_rand_string(12));
}
