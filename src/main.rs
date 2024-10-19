use image::{GenericImage, GenericImageView, Rgba};
use palette::{FromColor, Hsv, Srgb};

///normalazing pixel's rgb values
///param pixel: pixel from a photo
fn rgba_to_hsv(pixel: Rgba<u8>) -> Hsv {
    let srgb = Srgb::new(
        pixel.0[0] as f32 / 255.0,
        pixel.0[1] as f32 / 255.0,
        pixel.0[2] as f32 / 255.0,
    );
    Hsv::from_color(srgb)
}

///checks if pixel is representing a skin colour
fn check_if_is_skin(pixel: Hsv) -> bool {
    (pixel.hue.into_inner() < 0.1 || pixel.hue.into_inner() >= 0.9)
        && (pixel.saturation >= 0.2 && pixel.saturation <= 0.6)
        && pixel.value >= 0.4
}

fn main() {
    let paths = std::fs::read_dir("./res/").unwrap();
    for (counter, img_path) in paths.enumerate() {
        let mut img = image::open(img_path.unwrap().path()).expect("Failed to load an image");

        let (width, height) = (img.width(), img.height());
        for i in 0..width {
            for j in 0..height {
                let color = if check_if_is_skin(rgba_to_hsv(img.get_pixel(i, j))) {
                    Rgba([255, 255, 255, 255])
                } else {
                    Rgba([0, 0, 0, 255])
                };
                img.put_pixel(i, j, color);
            }
        }
        let save_path =
            String::from("target/images/modified_image") + &counter.to_string() + ".jpg";
        img.save(save_path).expect("failed to save an image");
    }
}
