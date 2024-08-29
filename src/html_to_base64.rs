use image::{DynamicImage, ImageFormat };
use base64::{encode, Engine};
use std::fs::File;
use std::io::Cursor;
use reqwest::Error;

pub async fn html_to_base64(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Load your HTML content (this example assumes you have an image in HTML)
    let html_content = get_html_page(url).await.expect("Htlm to parse failed at html content");

    // Extract the base64 string from the HTML content
    let base64_str = extract_base64_from_html(&html_content).expect("Htlm to parse failed at base64_str");

    // Decode the base64 string to get the image bytes
    let image_bytes = base64::decode(base64_str).expect("Htlm to parse failed at image_bytes");

    // Load the image from the bytes
    let img = image::load_from_memory(&image_bytes).expect("Htlm to parse failed at img");

    // Convert the image to PNG format and encode it to base64
    let mut buf = Vec::new();
    img.write_to(&mut Cursor::new(&mut buf), ImageFormat::Png).expect("Htlm to parse failed at img.write_to");
    let base64_encoded_image = encode(&buf);

    println!("Base64 Encoded Image: {}", &base64_encoded_image);

    Ok(base64_encoded_image)
}
fn extract_base64_from_html(html: &str) -> Result<&str, Box<dyn std::error::Error>> {
    // This is a simple example. You might need a more robust HTML parser.
    let start = html.find("base64,").ok_or("Base64 string not found").expect("Htlm to parse failed at image_bytes") + 7;
    let end = html[start..].find("'").ok_or("End of base64 string not found").expect("Htlm to parse failed at image_bytes") + start;
    Ok(&html[start..end])
}

async fn get_html_page(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await.expect("Htlm to parse failed at image_bytes");
    let html = response.text().await.expect("Htlm to parse failed at image_bytes");
    Ok(html)
}

