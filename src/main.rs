use std::io::prelude::*;

mod cursor;
mod graphics;

const X_SIZE: usize = 1280;
const Y_SIZE: usize = 800;
const ARR_SIZE: usize = X_SIZE*Y_SIZE*4;
const CURSOR_COLOR: graphics::Color = graphics::Color{r: 255, g: 255, b: 255, a: 255};
const CURSOR_RADIUS: f32 = 10.0;

fn main() {
    
    let mut arr: [u8; ARR_SIZE] = [0; ARR_SIZE];

    const TEST_IMAGE_WIDTH: u32 = 437;
    const TEST_IMAGE_HEIGHT: u32 = 438;
    
    let mut tb_vec: Vec<graphics::Thumbnail> = Vec::new();
    tb_vec.push(graphics::Thumbnail::new("/home/sacha/Documents/test/fb/testy.jpg", 0, 0));
    tb_vec.push(graphics::Thumbnail::new("/home/sacha/Documents/test/fb/testy.jpg", TEST_IMAGE_WIDTH, TEST_IMAGE_HEIGHT));
    tb_vec.push(graphics::Thumbnail::new("/home/sacha/Documents/test/fb/testy.jpg", TEST_IMAGE_WIDTH*2, TEST_IMAGE_HEIGHT));
    tb_vec.push(graphics::Thumbnail::new("/home/sacha/Documents/test/fb/testy.jpg", 0, TEST_IMAGE_HEIGHT*2));
    tb_vec.push(graphics::Thumbnail::new("/home/sacha/Documents/test/fb/testy.jpg", TEST_IMAGE_WIDTH*2, TEST_IMAGE_HEIGHT*2));
    tb_vec.push(graphics::Thumbnail::new("/home/sacha/Documents/test/fb/testy.jpg", TEST_IMAGE_WIDTH*3, TEST_IMAGE_HEIGHT*2));
    

    let mut cursor = cursor::Cursor::new(String::from("/dev/input/mice"), X_SIZE as i32, Y_SIZE as i32, CURSOR_RADIUS, CURSOR_COLOR);

    loop {
        cursor.update();
        
        let mut i = 0;
        while i < ARR_SIZE {
            let p = i / 4;
            let mut pixel = graphics::Pixel::new_black_pixel((p % X_SIZE) as u32, (p / X_SIZE) as u32);
           
            cursor.change_pixel_in_cursor(&mut pixel);

            for tb in &tb_vec {
                if tb.in_bounds(pixel.x, pixel.y){
                    
                    let img_pixel = tb.img.get_pixel(pixel.x-tb.x, pixel.y-tb.y);
                    pixel.set_bgra_color(img_pixel);
                }
            }
            
            arr[i] = pixel.color.b;
            arr[i+1] = pixel.color.g;
            arr[i+2] = pixel.color.r;
            arr[i+3] = pixel.color.a;
            i+=4;
        }

        let mut buffer = std::fs::OpenOptions::new().write(true).open("/dev/fb0").unwrap();
        buffer.write_all(&arr).unwrap();
    }
}