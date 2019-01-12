# Vigenère cipher
A Vigenère cipher implementation in Rust

## Example

```rs
let v = Vigenere::new();
let r = v.encrypt("saluttasdadereqrtttttttttt", "morqwewdti");
println!("{:?}", r); // "eockpxwvwipsiumvpwmbfhkjpx"
```