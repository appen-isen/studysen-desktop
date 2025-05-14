use aes_gcm::{
    aead::{Aead, Nonce},
    Aes256Gcm, KeyInit,
};
use rand::rngs::OsRng;
use rand::RngCore;
pub fn encrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    //On génère un nonce aléatoire de 12 octets
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::<Aes256Gcm>::from_slice(&nonce_bytes);

    // On chiffre les données
    let cipher = Aes256Gcm::new_from_slice(key).expect("Valid key length");
    let ciphertext = cipher.encrypt(nonce, data).expect("Encryption failed");

    // On combine le nonce et le ciphertext
    let mut result = Vec::with_capacity(nonce_bytes.len() + ciphertext.len());
    result.extend_from_slice(&nonce_bytes);
    result.extend_from_slice(&ciphertext);
    result
}

pub fn decrypt(data: &[u8], key: &[u8]) -> Option<Vec<u8>> {
    if data.len() < 12 {
        return None;
    }
    //On extrait le nonce des 12 premiers octets
    let (nonce_bytes, ciphertext) = data.split_at(12);
    let nonce = Nonce::<Aes256Gcm>::from_slice(nonce_bytes);

    let cipher = Aes256Gcm::new_from_slice(key).expect("Valid key length");
    cipher.decrypt(nonce, ciphertext).ok()
}
