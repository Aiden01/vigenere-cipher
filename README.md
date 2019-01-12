# Vigenère cipher
A Vigenère cipher implementation in Rust

## Example

```rs
let encryptor = Encryptor::new();
let r = encryptor.encrypt("saluttasdadereqrtttttttttt", "morqwewdti");
println!("{:?}", r); // "eockpxwvwipsiumvpwmbfhkjpx"
```