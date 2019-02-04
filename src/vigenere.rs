pub struct Vigenere;

impl Vigenere {
    pub fn new() -> Vigenere {
        Vigenere {}
    }

    /**
     * Encrypts a plain text with a key
     */
    pub fn encrypt<T>(&self, plain: T, k: T) -> String
    where
        T: Into<String>,
    {
        let text: String = plain.into();
        let key: String = self.set_key_len(&mut k.into(), text.len());
        let mut result = String::new();

        // Transform all the characters
        text.chars().zip(key.chars()).for_each(|(plain_c, key_c)| {
            let next_char =
                self.get_next_char(self.get_char_num(plain_c), self.get_char_num(key_c));
            result.push(next_char);
        });

        result
    }

    // /**
    //  * Decrypts a ciphertext
    //  */
    // pub fn decrypt<T>(&self, ciphertext: T, key: T) -> String
    // where
    //     T: Into<String>,
    // {
    //     let mut plain = String::new();
    //     let cipher: String = ciphertext.into();
    //     let mut k: String = key.into();

    //     if k.len() != cipher.len() {
    //         k = self.set_key_len(&mut k, cipher.len())
    //     }

    //     let mut i = 0;
    //     for c in cipher.chars() {
    //         i += 1;
    //         let key_num = self.get_char_num(k.chars().nth(i - 1).unwrap());
    //         let cipher_num = self.get_char_num(c);

    //         let char_pos = if key_num > cipher_num {
    //             26 + cipher_num - key_num
    //         } else {
    //             cipher_num - key_num
    //         };

    //         let plain_c = self.get_matched_char(char_pos);
    //         plain.push(plain_c);
    //     }

    //     plain
    // }

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
    fn get_next_char(&self, plain_num: usize, key_num: usize) -> char {
        let alphabet = String::from("abcdefghijklmnopqrstuvwxyz");
        let next_num = (plain_num + key_num) % 26;
        alphabet.chars().nth(next_num).unwrap()
    }
}
