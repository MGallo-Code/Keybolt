use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::sync::RwLock;

use secrets::SecretBytes;

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordEntry {
    pub title: String,
    pub username: String,
    pub password: SecretBytes,
}

pub struct SecureJsonIO {
    path: String,
    entries: RwLock<Vec<PasswordEntry>>,
}

impl SecureJsonIO {
    pub fn new(path: &str) -> Self {
        SecureJsonIO {
            path: path.to_string(),
            entries: RwLock::new(Vec::new()),
        }
    }

    pub fn load_entries(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Path::new(&self.path);
        if !path.exists() {
            return Ok(());
        }

        let mut file = File::open(path)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;

        let entries: Vec<PasswordEntry> = serde_json::from_slice(&contents)?;
        *self.entries.write().unwrap() = entries;

        Ok(())
    }

    pub fn save_entries(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Path::new(&self.path);
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;

        let entries = self.entries.read().unwrap();
        let json = serde_json::to_vec_pretty(&*entries)?;
        file.write_all(&json)?;

        Ok(())
    }

    pub fn get_entries(&self) -> Vec<PasswordEntry> {
        self.entries.read().unwrap().clone()
    }

    pub fn add_entry(&self, entry: PasswordEntry) {
        self.entries.write().unwrap().push(entry);
    }
}