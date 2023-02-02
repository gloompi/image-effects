use std::io::Cursor;

use image::{load_from_memory,ImageFormat::Png};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;
use base64::{Engine as _, engine::{general_purpose}};

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    let decoded_img = general_purpose::STANDARD
        .decode(encoded_file).unwrap();
    log(&"Encoded".into());

    let mut image = load_from_memory(&decoded_img).unwrap();
    image = image.grayscale();

    let mut buffer = Cursor::new(vec![]);
    
    image.write_to(&mut buffer, Png).unwrap();
    log(&"New image written".into());

    let encoded_img = general_purpose::STANDARD.encode(&buffer.into_inner());
    let data_url = format!(
        "data:image/png;base64,{}",
        encoded_img,
    );

    data_url
}