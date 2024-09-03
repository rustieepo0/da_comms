use aes_gcm::{Aes256Gcm, Key}; // Ensure this import matches your cipher choice
use aes_gcm::aead::{Aead, KeyInit, generic_array::GenericArray}; // Use KeyInit instead of NewAead
use rand::Rng;

pub fn encrypt_message(key: &[u8; 32], message: &str) -> Vec<u8> {
    let cipher = Aes256Gcm::new(GenericArray::from_slice(key)); // Use KeyInit's `new` method
    let nonce = rand::thread_rng().gen::<[u8; 12]>();
    let ciphertext = cipher.encrypt(GenericArray::from_slice(&nonce), message.as_bytes()).unwrap();
    [nonce.to_vec(), ciphertext].concat()
}

pub fn decrypt_message(key: &[u8; 32], ciphertext: &[u8]) -> String {
    let cipher = Aes256Gcm::new(GenericArray::from_slice(key)); // Use KeyInit's `new` method
    let nonce = GenericArray::from_slice(&ciphertext[..12]);
    let plaintext = cipher.decrypt(nonce, &ciphertext[12..]).unwrap();
    String::from_utf8(plaintext).unwrap()
}
