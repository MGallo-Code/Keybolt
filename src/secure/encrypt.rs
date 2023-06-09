use aes_gcm::aead::{generic_array::GenericArray, Aead};
use aes_gcm::Aes256Gcm;
use aes_gcm::KeyInit;
use argon2::Argon2;
use base64::{Engine};
use base64::engine::general_purpose;
use rand::{Rng, RngCore};
use secrets::SecretBox;
use secrets::traits::AsContiguousBytes;
use serde_json::{Error as SerdeError, json};
use serde_json::Value;
use sodiumoxide::utils;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::{BufWriter, Read, Write};
use std::string::FromUtf8Error;

use crate::gui::pages::details_page::EntryType;

pub fn encrypt_sensitive_fields(passphrase: &str, data: &mut Value, entry_type: EntryType) -> Result<(), EncryptError> {
    match entry_type {
        EntryType::Passwords => {
            data["password"] = encrypt_field(passphrase, data["password"].as_str().unwrap_or(""));
        }
        _ => ()
    }
    Ok(())
}

pub fn decrypt_sensitive_fields(passphrase: &str, data: &mut Value, entry_type: EntryType) -> Result<(), EncryptError> {
    match entry_type {
        EntryType::Passwords => {
            data["password"] = decrypt_field(passphrase, data["password"].as_str().unwrap_or(""));
        }
        _ => ()
    }
    Ok(())
}

fn encrypt_field(passphrase: &str, password_str: &str) -> Value {
    let salt = generate_salt();
    let key = derive_key(passphrase, &salt);
    let mut encrypted_password = encrypt(
        &key.borrow(),
        password_str.as_bytes(),
        &salt,
    ).unwrap();
    let password_data = general_purpose::STANDARD.encode(&encrypted_password.0);
    let password_nonce = general_purpose::STANDARD.encode(&encrypted_password.1);
    let password_salt = general_purpose::STANDARD.encode(&encrypted_password.2);

    let nonce_len = password_nonce.len();
    let salt_len = password_salt.len();

    let password_str = format!(
        "{}{}{}{}{}",
        nonce_len, salt_len, password_data, password_nonce, password_salt
    );
    
    utils::memzero(encrypted_password.0.as_mut_slice());
    utils::memzero(encrypted_password.1.as_mut_slice());
    utils::memzero(encrypted_password.2.as_mut_slice());
    
    return json!(password_str);
}

fn decrypt_field(passphrase: &str, password_str: &str) -> Value {
    let salt_offset = password_str.len() - &password_str[2..4].parse::<usize>().unwrap();
    let nonce_offset = salt_offset - &password_str[0..2].parse::<usize>().unwrap();

    let encrypted_data = general_purpose::STANDARD.decode(&password_str[4..nonce_offset]).unwrap();
    let nonce = general_purpose::STANDARD.decode(&password_str[nonce_offset..salt_offset]).unwrap();
    let salt = general_purpose::STANDARD.decode(&password_str[salt_offset..]).unwrap();

    let key = derive_key(passphrase, &salt.as_bytes());
    let decrypted_password =
        decrypt(&key.borrow(), &nonce, &encrypted_data).unwrap();

    return json!(String::from_utf8(decrypted_password).unwrap());
}

pub fn read_data(passphrase: &str) -> Result<Value, EncryptError> {
    let mut file = File::open("encrypted_data.bin")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let salt_offset = buffer.len() - 16;
    let nonce_offset = salt_offset - 12;

    let (ciphertext, rest) = buffer.split_at(nonce_offset);
    let (nonce, salt) = rest.split_at(12);

    let key = derive_key(&passphrase, &salt);

    let decrypted_data = decrypt(&key.borrow(), &nonce, &ciphertext);

    match decrypted_data {
        Ok(plaintext) => {
            let decrypted_data_str = String::from_utf8(plaintext)?;
            let decrypted_json: Value = serde_json::from_str(&decrypted_data_str)?;
            println!("Data decrypted successfully!");
            Ok(decrypted_json)
        }
        Err(e) => {
            eprintln!("Error decrypting data: {}", e);
            Err(e)
        }
    }
}

pub fn write_data(passphrase: &str, data: Value) -> Result<(), EncryptError> {
    let initial_salt = generate_salt();
    let key = derive_key(&passphrase, &initial_salt);

    let data_str = serde_json::to_string(&data)?;

    let encrypt_result = encrypt(&key.borrow(), data_str.as_bytes(), &initial_salt);

    match encrypt_result {
        Ok((ciphertext, nonce, salt)) => {
            // Write the ciphertext, nonce, and salt to a file
            let file = File::create("encrypted_data.bin")?;
            let mut buf_writer = BufWriter::new(file);
            buf_writer.write_all(&ciphertext).unwrap();
            buf_writer.write_all(&nonce).unwrap();
            buf_writer.write_all(&salt).unwrap();
            buf_writer.flush().unwrap();

            println!("Data encrypted successfully!");
        }
        Err(e) => {
            eprintln!("Error encrypting data: {}", e);
        }
    }

    Ok(())
}

pub fn derive_key(passphrase: &str, salt: &[u8]) -> SecretBox<[u8; 32]> {
    let key = SecretBox::new(|k: &mut [u8; 32]| {
        Argon2::default()
            .hash_password_into(passphrase.as_bytes(), &salt, k)
            .unwrap();
    });
    key
}

pub fn generate_salt() -> [u8; 16] {
    let mut salt = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut salt);
    salt
}

pub fn encrypt(
    key: &[u8; 32],
    data: &[u8],
    salt: &[u8],
) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>), EncryptError> {
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

// Custom error type that wraps the aes_gcm::Error, io::Error, serde_json::Error, and FromUtf8Error.
#[derive(Debug)]
pub enum EncryptError {
    Aes(aes_gcm::Error),
    Io(io::Error),
    Serde(SerdeError),
    Utf8(FromUtf8Error),
}

// Implement the From trait to convert aes_gcm::Error to the custom error type.
impl From<aes_gcm::Error> for EncryptError {
    fn from(err: aes_gcm::Error) -> EncryptError {
        EncryptError::Aes(err)
    }
}

// Implement the From trait to convert io::Error to the custom error type.
impl From<io::Error> for EncryptError {
    fn from(err: io::Error) -> EncryptError {
        EncryptError::Io(err)
    }
}

// Implement the From trait to convert serde_json::Error to the custom error type.
impl From<SerdeError> for EncryptError {
    fn from(err: SerdeError) -> EncryptError {
        EncryptError::Serde(err)
    }
}

// Implement the From trait to convert FromUtf8Error to the custom error type.
impl From<FromUtf8Error> for EncryptError {
    fn from(err: FromUtf8Error) -> EncryptError {
        EncryptError::Utf8(err)
    }
}

// Implement the Display trait for the custom error type.
impl fmt::Display for EncryptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EncryptError::Aes(e) => write!(f, "Encryption error: {}", e),
            EncryptError::Io(e) => write!(f, "IO error: {}", e),
            EncryptError::Serde(e) => write!(f, "Serde JSON error: {}", e),
            EncryptError::Utf8(e) => write!(f, "UTF-8 error: {}", e),
        }
    }
}
