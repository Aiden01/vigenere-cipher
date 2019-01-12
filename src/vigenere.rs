pub struct Vigenere;

impl Vigenere {
    pub fn new() -> Vigenere {
        Vigenere {}
    }

    /**
     * Encrypts a plain text with a key
     */
    pub fn encrypt<T>(&self, plain: T, key: T) -> String
    where
        T: Into<String>,
    {
        let text: String = plain.into();
        let mut k: String = key.into();
        let mut result = String::new();

        if k.len() != text.len() {
            k = self.set_key_len(&mut k, text.len())
        }

        let mut i = 0;
        for c in text.chars() {
            i += 1;
            let key_char = k.chars().nth(i - 1).unwrap();
            let key_num = self.get_char_num(key_char);
            let text_num = self.get_char_num(c);

            let cipher_char = self.get_matched_char(text_num + key_num);

            result.push(cipher_char);
        }

        result
    }

    /**
     * Decrypts a ciphertext
     */
    pub fn decrypt<T>(&self, ciphertext: T, key: T) -> String
    where
        T: Into<String>,
    {
        let mut plain = String::new();
        let cipher: String = ciphertext.into();
        let mut k: String = key.into();

        if k.len() != cipher.len() {
            k = self.set_key_len(&mut k, cipher.len())
        }

        let mut i = 0;
        for c in cipher.chars() {
            i += 1;
            let key_num = self.get_char_num(k.chars().nth(i - 1).unwrap());
            let cipher_num = self.get_char_num(c);

            let char_pos = if key_num > cipher_num {
                26 + cipher_num - key_num
            } else {
                cipher_num - key_num
            };

            let plain_c = self.get_matched_char(char_pos);
            plain.push(plain_c);
        }

        plain
    }

    /**
     * Sets the len of the key according to the len of the plain text
     */
    fn set_key_len(&self, k: &mut String, len: usize) -> String {
        while k.len() < len {
            k.push_str(&k.clone());
        }
        let temp = &k[0..len];
        String::from(temp)
    }

    fn get_char_num(&self, c: char) -> usize {
        let alphabet = String::from("abcdefghijklmnopqrstuvwxyz");
        alphabet.find(c).unwrap()
    }

    /**
     * Returns the matched char on the alphabet
     */
    fn get_matched_char(&self, i: usize) -> char {
        let alphabet = String::from("abcdefghijklmnopqrstuvwxyz");
        if i > 26 {
            alphabet.chars().nth(i - 26).unwrap()
        } else {
            alphabet.chars().nth(i).unwrap()
        }
    }
}
