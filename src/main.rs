// TOTP simple version
use hmac::{Hmac, Mac};
use sha1::Sha1;
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha1 = Hmac<Sha1>;

fn get_counter(time_step: u64) -> u64 {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    start / time_step
}

fn main() {
    let time_step = 30;

    let counter = get_counter(time_step);

    println!("Counter: {}", counter);
}
