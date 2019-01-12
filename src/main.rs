extern crate nalgebra;

mod encryptor;
use self::encryptor::Encryptor;

fn main() {
    let encryptor = Encryptor::new();
    let r = encryptor.encrypt("saluttasdadereqrtttttttttt", "morqwewdti");
    println!("{:?}", r);
}
