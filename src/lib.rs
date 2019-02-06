pub mod vigenere;
pub use self::vigenere::Vigenere;



#[test]

fn encrypt_string() {
    let vigenere = Vigenere::new();
    let plain = "lemon";
    let key = "computer";
    assert_eq!(vigenere.encrypt(plain, key), "nsydh");
}

#[test]
fn decrypt_string() {
    let vigenere = Vigenere::new();
    let cipher_text = "nsydh";
    let key = "computer";
    assert_eq!(vigenere.decrypt(cipher_text, key), "lemon");
}
