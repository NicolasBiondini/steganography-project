mod steganography;

use std::env;
use steganography::{hide_message, retrieve_message};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let image_path = &args[1];

    if args.len() == 2 {
        // One argument provided, retrieve the message
        let modified_image = image::open(image_path).expect("Failed to open modified image");
        let retrieved_message = retrieve_message(&modified_image.to_rgba8(),128);
        println!("Retrieved message: {}", retrieved_message);
        return;
    }

    let key = &args[2];

    if args.len() == 3 {
        // Only two arguments provided, hide the hash
        let mut image = image::open(image_path)
        .expect("Failed to open image")
        .to_rgba8();

        hide_message(&mut image, key);
        let output_path = "output.png"; // Replace with your output image path
        image.save(output_path).expect("Failed to save image");
        println!("Message hidden in the image.");
    } else {
        println!("Invalid number of arguments.");
    }
}

