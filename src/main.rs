use image::{GenericImage, GenericImageView, Rgba};
use palette::{FromColor, Hsv, Srgb};

///normalazing pixel's rgb values
///param pixel: pixel from a photo
fn rgba_to_hsv(pixel: Rgba<u8>) -> Hsv {
    //normalazing rgb colors and saving them to Srgb wrapper
    let srgb = Srgb::new(
        pixel.0[0] as f32 / 255.0,
        pixel.0[1] as f32 / 255.0,
        pixel.0[2] as f32 / 255.0,
    );
    Hsv::from_color(srgb) //conversion from rgb to hsv
}

///checks if pixel is representing a skin colour
fn check_if_is_skin(pixel: Hsv) -> bool {
    (pixel.hue.into_inner() < 0.1 || pixel.hue.into_inner() >= 0.9) 
        && (pixel.saturation >= 0.2 && pixel.saturation <= 0.6)
        && pixel.value >= 0.4
}

fn main() {
    let img_path="res/dlon.jpg"; //path to image
    let mut img = image::open(img_path).expect("Failed to load an image");//loading image with rgba values

    let (width, height) = (img.width(), img.height()); //getting image size

    let mut x_cords = Vec::new();
    let mut y_cords=Vec::new();
    //iterating over every pixel in an image
    for i in 0..width {
        for j in 0..height {
            //changing pixels color to black if not skin or to white if skin 
            let color = if check_if_is_skin(rgba_to_hsv(img.get_pixel(i, j))) {
                x_cords.push(i);
                y_cords.push(j);
                Rgba([255, 255, 255, 255])
            } else {
                Rgba([0, 0, 0, 255])
            };
            img.put_pixel(i, j, color); //saving pixel with new color
        }
    }
    let x_sum: u32=x_cords.iter().sum();
    let x_mean=x_sum/x_cords.len() as u32;
    let y_sum: u32=y_cords.iter().sum();
    let y_mean=y_sum/y_cords.len() as u32;

    for x in x_mean-3..x_mean+3  {
        for y in y_mean-3..y_mean+3  {
            img.put_pixel(x, y, Rgba([255,0,0,255]));
        }
    }

    
    let save_path="./target/images/modified_image1.jpg"; //path to saved image
    img.save(save_path).expect("failed to save an image"); //saving modified image
}
