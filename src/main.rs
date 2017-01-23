extern crate base32;
extern crate oath;

static SECRET: &'static str = env!("SECRET");

fn main() {
    let length = 6;
    let offset = 0;
    let steps = 30;

    let key = base32::decode(base32::Alphabet::RFC4648 {padding: false}, SECRET).unwrap();
    println!("{}", oath::totp_raw(key.as_ref(), length, offset, steps));
}
