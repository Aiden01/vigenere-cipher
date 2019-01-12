# Vigenère cipher
A Vigenère cipher implementation in Rust

## Example

```rs
let v = Vigenere::new();
let r = v.encrypt("saluttasdadereqrtttttttttt", "morqwewdti");
println!("Encrypted {:?}", r);
let r2 = v.decrypt(r, "morqwewdti".to_string());
println!("Decrypted {:?}", r2); // "saluttasdadereqrtttttttttt"
```