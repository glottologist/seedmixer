use crate::errors::{AsciiError, SeedMixerError};
use colored::*;
use figlet_rs::FIGfont;
use image::DynamicImage;

// Function to load an image from byte array and convert it into a DynamicImage
fn load_image_from_bytes(bytes: &[u8]) -> Result<DynamicImage, SeedMixerError> {
    // Attempt to load an image from memory, map any error to a custom SeedMixerError
    Ok(image::load_from_memory(bytes).map_err(AsciiError::UnableToLoadAsciiArtImageFromMemory)?)
}

// Function to generate ASCII art representation of the provided image
fn generate_logo_ascii(img: DynamicImage) -> Result<String, SeedMixerError> {
    // Configure ASCII art generation with an outline
    let builder = artem::ConfigBuilder::new()
        .border(true)
        .invert(true)
        .outline(false)
        .scale(0.5f32)
        .hysteresis(true)
        .characters(" #@&.*".to_string())
        .build();

    // Convert the image to ASCII art using the specified configuration
    Ok(artem::convert(img, &builder))
}

// Public function to generate a terminal header from a predefined logo image
pub fn generate_terminal_header() -> Result<(), SeedMixerError> {
    // Embedded logo image as a byte array
    const MIXER_LOGO: &[u8] = include_bytes!("../data/mixer_logo.png");
    // Load the logo image into a DynamicImage object
    let img = load_image_from_bytes(MIXER_LOGO)?;
    // Generate ASCII art from the loaded image
    let ascii_art = generate_logo_ascii(img)?;

    // Print the ASCII art in green color to the terminal
    println!("{}", ascii_art.green());

    let font = FIGfont::standard().unwrap();
    let figure = font.convert("SEEDMIXER");

    if let Some(figure) = figure {
        println!("{}", figure);
    }
    // Return Ok to indicate success
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::{AsciiError, SeedMixerError};
    use proptest::prelude::*;

    // A very small 1x1 PNG (fake or real) just to test loading
    const TEST_PNG: &[u8] = include_bytes!("../data/test.png");

    #[test]
    fn test_load_image_from_bytes_valid() {
        let result = load_image_from_bytes(TEST_PNG);
        assert!(result.is_ok(), "Expected to successfully load the image");
        let img = result.unwrap();
        // We can check basic properties (e.g., width/height)
        assert_eq!(img.width(), 1);
        assert_eq!(img.height(), 1);
    }

    #[test]
    fn test_load_image_from_bytes_invalid() {
        let invalid_data = &[0, 1, 2, 3];
        let result = load_image_from_bytes(invalid_data);
        match result {
            Err(SeedMixerError::Ascii(AsciiError::UnableToLoadAsciiArtImageFromMemory(_))) => {
                // This is the expected error
            }
            _ => panic!("Expected UnableToLoadAsciiArtImageFromMemory error."),
        }
    }

    #[test]
    fn test_generate_logo_ascii() {
        // Create a 1x1 image in memory (using, e.g., `image::DynamicImage::new_rgb8`)
        let img = image::DynamicImage::new_rgb8(1, 1);
        let result = generate_logo_ascii(img);
        assert!(result.is_ok(), "ASCII generation should succeed");
        // We can examine the resulting string if needed
        let ascii = result.unwrap();
        assert!(!ascii.is_empty(), "ASCII result should not be empty");
    }

    // If your code can be tested with the real embedded image, you can do:
    #[test]
    fn test_generate_terminal_header() {
        // This test calls the full pipeline
        let result = generate_terminal_header();
        assert!(
            result.is_ok(),
            "Should generate and print the header successfully"
        );
    }

    proptest! {
        // Generate random small widths and heights
        #[test]
        fn test_generate_logo_ascii_random(width in 1..10u32, height in 1..10u32) {
            // Create a random image with the given dimensions
            let mut img = image::ImageBuffer::new(width, height);
            for pixel in img.pixels_mut() {
                // Assign random color channels
                *pixel = image::Rgb([
                    rand::random::<u8>(),
                    rand::random::<u8>(),
                    rand::random::<u8>()
                ]);
            }
            let dynamic_image = image::DynamicImage::ImageRgb8(img);

            // Convert to ASCII
            let ascii_result = generate_logo_ascii(dynamic_image);
            prop_assert!(ascii_result.is_ok(), "ASCII art conversion should not fail");
            let ascii_art = ascii_result.unwrap();
            // Just check it’s non-empty
            prop_assert!(!ascii_art.is_empty());
        }
    }
}
