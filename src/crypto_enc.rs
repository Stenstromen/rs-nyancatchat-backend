// src/crypto_enc.rs

use aes::Aes256;
use ctr::{Ctr64BE, cipher::{NewCipher, StreamCipher}};
use hex;
use rand::RngCore;

type Aes256Ctr = Ctr64BE<Aes256>;

pub fn encrypt(secret_key: &str, plaintext: &str) -> (String, String) {
    let key = secret_key.as_bytes();
    let mut iv = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut iv);

    let cipher = Aes256Ctr::new_from_slices(key, &iv).unwrap();
    let ciphertext = cipher.encrypt_vec(plaintext.as_bytes());

    (hex::encode(iv), hex::encode(ciphertext))
}

pub fn decrypt(secret_key: &str, iv_hex: &str, ciphertext_hex: &str) -> String {
    let key = secret_key.as_bytes();
    let iv = hex::decode(iv_hex).unwrap();
    let ciphertext = hex::decode(ciphertext_hex).unwrap();

    let cipher = Aes256Ctr::new_from_slices(key, &iv).unwrap();
    let decrypted_ciphertext = cipher.decrypt_vec(&ciphertext).unwrap();

    String::from_utf8(decrypted_ciphertext).unwrap()
}