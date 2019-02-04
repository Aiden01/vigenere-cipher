pub mod vigenere;
pub use self::vigenere::Vigenere;



#[test]

fn encrypt_string() {
    let vigenere = Vigenere::new();
    let plain = "lemon";
    let key = "computer";
    assert_eq!(vigenere.encrypt(plain, key), "nsydh");
}
