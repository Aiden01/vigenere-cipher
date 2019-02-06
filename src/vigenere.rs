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
            let next_char = self.get_char_num(plain_c) + self.get_char_num(key_c);
            result.push(self.get_next_char(next_char));
        });

        result
    }

    /**
     * Decrypts a ciphertext
     */
    pub fn decrypt<T>(&self, ciphertext: T, k: T) -> String
    where
        T: Into<String>,
    {
        let cipher: String = ciphertext.into();
        let key: String = self.set_key_len(&mut k.into(), cipher.len());
        let mut plain = String::new();

        // Transform all the characters
        cipher
            .chars()
            .zip(key.chars())
            .for_each(|(plain_c, key_c)| {
                let key_num = self.get_char_num(key_c);
                let plain_num = self.get_char_num(plain_c);
                let next_num = if plain_num >= key_num {
                    plain_num - key_num
                } else {
                    key_num - plain_num
                };
                plain.push(self.get_next_char(next_num));
            });

        plain
    }

    /**
     * Sets the len of the key according to the len of the plain text
     */
    fn set_key_len(&self, k: &mut String, len: usize) -> String {
        k.chars().cycle().take(len).collect()
    }

    fn get_char_num(&self, c: char) -> u8 {
        c as u8 - 'a' as u8
    }

    /**
     * Returns the matched char on the alphabet
     */
    fn get_next_char(&self, n: u8) -> char {
        let alphabet = String::from("abcdefghijklmnopqrstuvwxyz");
        let next_num = n % 26;
        alphabet.chars().nth(next_num as usize).unwrap()
    }
}
