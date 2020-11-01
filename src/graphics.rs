use image::io::Reader as ImageReader;

#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub struct Pixel{
    pub color: Color,
    pub x: u32,
    pub y: u32,
}

impl Pixel{

    pub fn new_black_pixel(x: u32, y:u32) -> Pixel{
        Pixel{
            color: Color{
                r: 0, g: 0, b: 0, a:0
            },
            x: x,
            y: y
        }
    } 
    pub fn set_bgra_color(&mut self, bgra: &image::Bgra<u8>){
        self.color.b = bgra[0];
        self.color.g = bgra[1];
        self.color.r = bgra[2];
        self.color.a = bgra[3];
    }
}

// x,y are the coordinates of the top left corner
pub struct Thumbnail{
    pub img: image::ImageBuffer<image::Bgra<u8>, std::vec::Vec<u8>>,
    width: u32,
    height: u32,
    pub x: u32,
    pub y: u32

}

impl Thumbnail{
    pub fn new(path: &str, x_arg:u32, y_arg:u32)->Thumbnail{
        let image = ImageReader::open(path).unwrap().decode().unwrap().to_bgra();
        let w = image.width();
        let h = image.height();
        Thumbnail{
            img: image,
            width: w,
            height: h,
            x: x_arg,
            y: y_arg
        }

    }

    // Check if the given pixel coordinates are in the image
    pub fn in_bounds(&self, x: u32, y:u32) -> bool {

        // If x is before the image
        if x < self.x {
            return false;
        }

        if y < self.y {
            return false;
        }

        // if x is after the last pixel
        if x > (self.x + self.width-1) {
            return false;
        }

        if y > (self.y + self.height-1){
            return false;
        }

        true
    }
    
}