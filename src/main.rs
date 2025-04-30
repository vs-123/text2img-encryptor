extern crate image;

use std::io::{stdin, stdout, Write};

fn cmd_input(msg: &str) -> String {
    let mut s = String::new();
    print!("{}", msg);
    let _ =stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    s.trim_end().to_string()
}

fn encrypt(msg: String, output_file_name: String) {
    let imgx = 500;
    let imgy = 500;

    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    let msg = msg.chars().map(|c| c as u8).collect::<Vec<u8>>();
    let msg_len = msg.len();

    for (i, (_x, _y, pixel)) in imgbuf.enumerate_pixels_mut().enumerate() {
        if i+1 > msg_len {
            break;
        }
        let character = msg.clone()[i as usize];
        *pixel = image::Rgb([character, character, character]);
    }

    let mut output_file_name = output_file_name;

    if !output_file_name.ends_with(".png") {
        output_file_name += ".png";
    }

    imgbuf.save(output_file_name).unwrap();
}

fn decrypt(image_file_name: String) -> Result<String, String> {
    let mut decrypted_msg = String::new();
    match image::open(image_file_name) {
        Ok(img) => {
            if let image::DynamicImage::ImageRgb8(imgbuf) = img {
                for (_x, _y, pixel) in imgbuf.enumerate_pixels() {
                    let pixel_0_value = pixel.0[0];
                    if pixel_0_value != 0 {
                        decrypted_msg.push(pixel_0_value as char)
                    }
                }
            }
        }

        Err(_) => {
            return Err("Error: Could not open file.".to_string());
        }
    }

    Ok(decrypted_msg)
}

fn main() {
    loop {
        let mode = cmd_input("Enter the mode [encrypt/decrypt]: ");
        let mode = mode.as_str();

        match mode {
            "encrypt" => {
                let msg = cmd_input("Enter the message to encrypt: ");
                let output_file_name = cmd_input("Enter the output file name (only .png files): ");
                encrypt(msg, output_file_name);
                println!("Success!");
            }

            "decrypt" => {
                let image_file_name = cmd_input("Enter the image file to decrypt: ");

                match decrypt(image_file_name) {
                    Ok(decrypted_msg) => println!("Decrypted: {}", decrypted_msg),
                    Err(e) => println!("{}", e),
                }
            }

            _ => ()
        }
    }
}
