pub fn encrypt(s: &str, shift: u32) -> String {
   let mut cipher_text = String::new(); 
   for l in s.chars() {
        cipher_text.push(char::from_u32((l as u32) + shift).unwrap());
    } 

    cipher_text
}

pub fn decrypt(s: &str, shift: u32) -> String {
   let mut cipher_text = String::new(); 
   for l in s.chars() {
        cipher_text.push(char::from_u32((l as u32) - shift).unwrap());
    } 

    cipher_text
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ecrypt() {
        assert_eq!("dbu", encrypt("cat", 1));
    }

    #[test]
    fn encrypt_decrypt() {
        assert_eq!("cat", decrypt("dbu", 1))
    }
}
