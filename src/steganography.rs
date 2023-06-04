use image::RgbaImage;
use sha2::{Digest, Sha512};
use hex;

// Make the hash based on the image and an external key
fn compute_image_hash(image: &RgbaImage, key: &str) -> String {
    let mut hasher = Sha512::new();

    // Hash the image pixels
    for pixel in image.pixels() {
        let pixel_data = pixel.0;
        hasher.update(&pixel_data);
    }

    // Hash the key
    hasher.update(key.as_bytes());

    let hash = hasher.finalize();
    let hash_string = hex::encode(hash);

    println!("Generated Hash: {}", hash_string);

    hash_string
}

pub fn hide_message(image: &mut RgbaImage, key: &str) {
    // Generate SHA-512 hash based on the image and key
    let image_hash = compute_image_hash(image, key);

    let bytes = image_hash.as_bytes();

    let mut byte_index = 0;
    let mut bit_index = 0;

    for pixel in image.pixels_mut() {
        let channel = &mut pixel.0[2]; // Manipulating the blue channel (LSB)

        if byte_index < bytes.len() {
            let bit = (bytes[byte_index] >> bit_index) & 1;
            *channel = (*channel & 0xFE) | bit; // Set the LSB of the blue channel to the message bit

            bit_index += 1;
            if bit_index == 8 {
                byte_index += 1;
                bit_index = 0;
            }
        } else {
            // If the entire message has been embedded, continue iterating over the pixels
            // but stop modifying the image
            break;
        }
    }
}

pub fn retrieve_message(image: &RgbaImage, hash_length: usize) -> String {
    let mut bytes: Vec<u8> = Vec::new();

    let mut byte: u8 = 0;
    let mut bit_index = 0;
    let mut byte_count = 0;

    for pixel in image.pixels() {
        let channel = pixel.0[2]; // Retrieving the blue channel (LSB)

        byte |= (channel & 1) << bit_index;

        bit_index += 1;
        if bit_index == 8 {
            bytes.push(byte);
            byte = 0;
            bit_index = 0;
            byte_count += 1;

            if byte_count == hash_length {
                // If the expected number of bytes has been retrieved, stop iterating over the pixels
                break;
            }
        }
    }
    
    String::from_utf8_lossy(&bytes).to_string()

}
