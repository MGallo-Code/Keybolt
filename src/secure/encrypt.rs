// encrypt.rs

// Import necessary traits and structs from the `aes_gcm`, `rand`, and `argon2` crates.
use aes_gcm::aead::{Aead, generic_array::GenericArray};
use aes_gcm::Aes256Gcm;
use aes_gcm::KeyInit;
use argon2::Argon2;
use rand::{
    Rng,
    RngCore,
};
use serde_json::Value;
use zeroize::Zeroize;
use std::fmt;
use std::fs::File;
use std::io::{Read, BufWriter, Write};

pub fn read_data(passphrase: String) -> Value {
    let initial_salt = generate_salt();
    let mut key = derive_key(&passphrase, &initial_salt);

    let mut file = File::open("encrypted_data.bin").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let salt_offset = buffer.len() - 16;
    let nonce_offset = salt_offset - 12;

    let (ciphertext, rest) = buffer.split_at(nonce_offset);
    let (nonce, _salt) = rest.split_at(12);

    let decrypted_data = decrypt(&key, &nonce, &ciphertext).unwrap();
    let decrypted_data_str = String::from_utf8(decrypted_data).unwrap();
    let decrypted_json: Value = serde_json::from_str(&decrypted_data_str).unwrap();

    println!("Data decrypted successfully!");

    key.zeroize();
    
    decrypted_json
}

pub fn save_data(passphrase: String, data: Value) {
    let initial_salt = generate_salt();
    let mut key = derive_key(&passphrase, &initial_salt);

    let mut file = File::open("encrypted_data.bin").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let data_str = serde_json::to_string(&data).unwrap();
    let (ciphertext, nonce, salt) = encrypt(&key, data_str.as_bytes(), &initial_salt).unwrap();
    
    // Write the ciphertext, nonce, and salt to a file
    let file = File::create("encrypted_data.bin").unwrap();
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write_all(&ciphertext).unwrap();
    buf_writer.write_all(&nonce).unwrap();
    buf_writer.write_all(&salt).unwrap();
    buf_writer.flush().unwrap();

    println!("Data decrypted successfully!");

    key.zeroize();
}

pub fn derive_key(passphrase: &str, salt: &[u8]) -> [u8; 32] {
    let mut key = [0u8; 32];
    Argon2::default().hash_password_into(passphrase.as_bytes(), salt, &mut key).unwrap();
    key
}

pub fn generate_salt() -> [u8; 16] {
    let mut salt = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut salt);
    salt
}

pub fn encrypt(key: &[u8; 32], data: &[u8], salt: &[u8]) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>), EncryptError> {
    let cipher = Aes256Gcm::new(GenericArray::from_slice(key));
    let nonce = generate_nonce();
    let ciphertext = cipher.encrypt(GenericArray::from_slice(&nonce), data)?;
    Ok((ciphertext, nonce.to_vec(), salt.to_vec())) // Return the salt as well
}

pub fn decrypt(key: &[u8; 32], nonce: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, EncryptError> {
    let cipher = Aes256Gcm::new(GenericArray::from_slice(key));
    let plaintext = cipher.decrypt(GenericArray::from_slice(nonce), ciphertext)?;
    Ok(plaintext)
}

// Function to generate a 96-bit (12-byte) random nonce for encryption.
fn generate_nonce() -> [u8; 12] {
    let mut nonce = [0u8; 12];
    // Fill the nonce with random bytes using a cryptographically secure random number generator.
    rand::thread_rng().fill(&mut nonce);
    nonce
}

// Custom error type that wraps the `aes_gcm::Error`.
#[derive(Debug)]
pub struct EncryptError(aes_gcm::Error);

// Implement the `Display` trait for the custom error type.
impl fmt::Display for EncryptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Encryption error: {}", self.0)
    }
}

// Implement the `std::error::Error` trait for the custom error type.
impl std::error::Error for EncryptError {}

// Implement the `From` trait to convert `aes_gcm::Error` to the custom error type.
impl From<aes_gcm::Error> for EncryptError {
    fn from(err: aes_gcm::Error) -> EncryptError {
        EncryptError(err)
    }
}
