extern crate crypto;

use crypto::aead::{AeadDecryptor, AeadEncryptor};
use crypto::chacha20::ChaCha20;
use crypto::symmetriccipher::SynchronousStreamCipher;

pub fn encrypt_chacha20(key: &[u8; 32], nonce: &[u8; 8], data: &mut [u8]) {
    let mut chacha20 = ChaCha20::new(key, nonce);

    // Create a separate buffer for the output
    let mut output = vec![0; data.len()];

    // Process the data in chunks of 64 bytes (ChaCha20 block size)
    for (chunk_in, chunk_out) in data.chunks(64).zip(output.chunks_mut(64)) {
        chacha20.process(chunk_in, chunk_out);
    }

    // Copy the encrypted data back to the original buffer
    data.copy_from_slice(&output);
}

pub fn decrypt_chacha20(key: &[u8; 32], nonce: &[u8; 8], data: &mut [u8]) {
    let mut chacha20 = ChaCha20::new(key, nonce);

    // Create a separate buffer for the output
    let mut output = vec![0; data.len()];

    // Process the data in chunks of 64 bytes (ChaCha20 block size)
    for (chunk_in, chunk_out) in data.chunks(64).zip(output.chunks_mut(64)) {
        chacha20.process(chunk_in, chunk_out);
    }

    // Copy the decrypted data back to the original buffer
    data.copy_from_slice(&output);
}
