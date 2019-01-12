extern crate nalgebra;

mod vigenere;
use self::vigenere::Vigenere;

fn main() {
    let v = Vigenere::new();
    let r = v.encrypt("saluttasdadereqrtttttttttt", "morqwewdti");
    println!("{:?}", r);
}
