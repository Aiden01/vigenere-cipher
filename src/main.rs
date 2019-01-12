mod vigenere;
use self::vigenere::Vigenere;

fn main() {
    let v = Vigenere::new();
    let r = v.encrypt("saluttasdadereqrtttttttttt", "morqwewdti");
    println!("Encrypted {:?}", r);
    let r2 = v.decrypt(r, "morqwewdti".to_string());
    println!("Decrypted {:?}", r2); // "saluttasdadereqrtttttttttt"
}
