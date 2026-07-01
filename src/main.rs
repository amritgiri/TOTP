// TOTP simple version
use hmac::{Hmac, Mac};
use sha1::Sha1;
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha1 = Hmac<Sha1>;

fn get_counter(time_step: u64) -> u64 {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    println!("Current time (seconds since epoch): {}", start);

    start / time_step
}

fn decode_secret(secret: &str) -> Vec<u8> {
    todo!()
}

fn generate_hmac(key: &[u8], counter: u64) -> Vec<u8> {
    todo!()
}

fn truncate(hmac: &[u8]) -> u32 {
    todo!()
}

fn generate_totp(secret: &str) -> u32 {
    todo!()
}

fn main() {
    let time_step = 30;

    let counter = get_counter(time_step);

    println!("Counter: {}", counter);
}
