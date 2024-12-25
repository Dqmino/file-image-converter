use num_traits::real::Real;
use std::fs;
use image::{ImageBuffer, Rgb};

fn read_file_as_3bytes_chunks(path: &str) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {
    let byte_content = fs::read(path)?;
    let result = byte_content.chunks(3).map(|chunk| chunk.to_vec()).collect();
    Ok(result)
}
pub fn convert_pixel_rgb_bytes_to_png(data: Vec<Vec<u8>>, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let data_size = data.len() as u32;
    let image_length = f32::sqrt(data_size as f32).ceil() as u32;
    let mut image = ImageBuffer::new(image_length, image_length);
    for (x, y, pixel) in image.enumerate_pixels_mut() {
            let pixel_index = (y * image_length + x) as usize;
            if pixel_index >= data.len() {
                continue;
            }
            let red = data[pixel_index][0];
            let green = match data[pixel_index].len() {
                1 => 0,
                _ => data[pixel_index][1]
            };
            let blue = match data[pixel_index].len() {
                1 => 0,
                2 => 0,
                _ => data[pixel_index][2]
            };
            *pixel = Rgb([red, green, blue]);
    }
    image.save(output_path)?;
    Ok(())
}
pub fn convert_file_to_png(path: &str, output_path: &str) -> Result<(Vec<Vec<u8>>), Box<dyn std::error::Error>> {
    let data = read_file_as_3bytes_chunks(path)?;
    convert_pixel_rgb_bytes_to_png(data.clone(), output_path)?;
    Ok(data)
}

pub fn convert_png_to_file(path_to_image: &str, path_to_output: Option<&str>) -> std::io::Result<Vec<u8>> {
    let image = image::open(path_to_image).unwrap();
    let data = image.into_rgb8();
    let mut byte_content: Vec<u8> = vec![];
    data.enumerate_pixels().for_each(|(_, _, pixel)| {
        byte_content.push(pixel.0[0]);
        byte_content.push(pixel.0[1]);
        byte_content.push(pixel.0[2]);
    });
    if path_to_output.is_none() {
        return Ok(byte_content);
    }
    fs::write(path_to_output.unwrap(), byte_content.clone())?;
    Ok(byte_content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_file_to_png() {
        let pixel_rgb_bytes = convert_file_to_png("C:\\Users\\user\\file.txt", "C:\\Users\\user\\file-image.png").unwrap();
        let data = convert_png_to_file("C:\\Users\\user\\file-image.png", Some("C:\\Users\\user\\file-text.txt")).unwrap();

        assert_eq!(clean_up(data), join(pixel_rgb_bytes));
    }

    // function to join nested vectors into a single vector
    fn join(data: Vec<Vec<u8>>) -> Vec<u8> {
        data.into_iter().flat_map(|chunk| chunk).collect()
    }

    // function to clean up excess zeroes
    fn clean_up(data: Vec<u8>) -> Vec<u8> {
        data.into_iter().filter(|&byte| byte != 0).collect()
    }
}

