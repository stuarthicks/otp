extern crate base32;
extern crate oath;

use oath::{totp_raw_now, HashType};

static SECRET: &'static str = env!("SECRET");

fn main() {
    let key = base32::decode(base32::Alphabet::RFC4648 { padding: false }, SECRET);
    match key {
        Some(v) => println!("{:?}", totp_raw_now(v.as_ref(), 6, 0, 30, &HashType::SHA1)),
        None => println!("Failed to base32 decode the totp secret"),
    }
}
