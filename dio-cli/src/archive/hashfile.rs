use anyhow::anyhow;
use chacha20poly1305::{aead::Aead, consts::U24, ChaChaPoly1305, KeyInit, XChaCha20Poly1305};
use rand::{rngs::OsRng, RngCore};
use std::fs;

/// See https://github.com/skerkour/kerkour.com/blob/main/2021/rust_file_encryption/src/main.rs
pub fn encrypt_decrypt() {
    let mut small_file_key = [0u8; 32];
    let mut small_file_nonce = [0u8; 24];
    OsRng.fill_bytes(&mut small_file_key);
    OsRng.fill_bytes(&mut small_file_nonce);
    // dbg!(&small_file_key, &small_file_nonce);
    println!("Encrypting facts.json to facts.encrypted");
    DioChaCha::encrypt_small_file(
        "facts.json",
        "facts.encrypted",
        &small_file_key,
        &small_file_nonce,
    )
    .unwrap();
    println!("Decrypting facts.encrypted to facts.json");
    DioChaCha::decrypt_small_file(
        "facts.encrypted",
        "facts.json",
        &small_file_key,
        &small_file_nonce,
    )
    .unwrap();
}

#[derive(Debug)]
struct DioChaCha;

impl DioChaCha {
    /// #[rustfmt::skip]
    /// See https://kerkour.com/rust-file-encryption
    /// https://github.com/skerkour/kerkour.com/blob/main/2021/rust_file_encryption
    fn encrypt_small_file(
        filepath: &str,
        dist: &str,
        key: &[u8; 32],
        nonce: &[u8; 24],
    ) -> Result<(), anyhow::Error> {
        let cipher: ChaChaPoly1305<_, U24> = XChaCha20Poly1305::new(key.into());

        let file_data = fs::read(filepath)?;

        let encrypted_file: Vec<u8> = cipher
            .encrypt(nonce.into(), file_data.as_ref())
            .map_err(|err| anyhow!("Encrypting small file: {}", err))?;

        fs::write(dist, encrypted_file)?;

        Ok(())
    }

    fn decrypt_small_file(
        encrypted_file_path: &str,
        dist: &str,
        key: &[u8; 32],
        nonce: &[u8; 24],
    ) -> Result<(), anyhow::Error> {
        let cipher: ChaChaPoly1305<_, U24> = XChaCha20Poly1305::new(key.into());
        // `.into()` :: Calls U::from(self).  // That is, this conversion is whatever the implementation of
        let file_data_ciphertext = fs::read(encrypted_file_path)?;

        let decrypted_file = cipher
            .decrypt(nonce.into(), file_data_ciphertext.as_ref())
            .map_err(|err| anyhow!("Decrypting small file: {}", err))?;

        fs::write(dist, decrypted_file)?;

        Ok(())
    }
}
