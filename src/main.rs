mod chacha20;

use gtk::prelude::*;
use gtk::{Button, FileChooserAction, FileChooserDialog, Label, ResponseType, Window, WindowType};
use std::fs::File;
use std::io::{Read, Write};

fn read_file(path: &str) -> Vec<u8> {
    let mut file = File::open(path).expect("Unable to open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Unable to read file");
    buffer
}

fn write_file(path: &str, data: &[u8]) {
    let mut file = File::create(path).expect("Unable to create file");
    file.write_all(data).expect("Unable to write to file");
}

fn main() {
    // Replace these with your actual key and nonce
    let key = [0u8; 32];
    let nonce = [0u8; 8];

    // gtk initialization
    gtk::init().expect("Failed to initialize GTK.");

    // create a new top level window
    let window = Window::new(WindowType::Toplevel);
    window.set_title("File Encryption tool");
    window.set_default_size(400, 200);

    // create label and button
    let input_label = Label::new(Some("Input File:"));
    let output_label = Label::new(Some("Output File:"));
    let encrypt_button = Button::with_label("Encrypt:");
    let decrypt_button = Button::with_label("Decrypt:");

    // Example usage of the ChaCha20 encryption function
    let input_path = "/home/crestfallen/Documents/secure-file-encryption/public/pfp.jpeg";
    let output_path = "/home/crestfallen/Documents/secure-file-encryption/encrypted/pfp.jpeg";

    let mut data_to_encrypt = read_file(input_path);
    chacha20::encrypt_chacha20(&key, &nonce, &mut data_to_encrypt);
    write_file(output_path, &data_to_encrypt);

    // Example usage of the ChaCha20 decryption function
    let input_path = "/home/crestfallen/Documents/secure-file-encryption/encrypted/pfp.jpeg";
    let output_path = "/home/crestfallen/Documents/secure-file-encryption/decrypted/pfp.jpeg";

    let mut data_to_decrypt = read_file(input_path);
    chacha20::decrypt_chacha20(&key, &nonce, &mut data_to_decrypt);
    write_file(output_path, &data_to_decrypt);
}
