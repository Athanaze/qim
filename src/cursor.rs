use std::fs::File;
use std::io;
use std::io::prelude::*;
use crate::graphics;

pub struct Cursor {
    pub x: i32,
    pub y: i32,
    f: std::fs::File,
    buffer: [u8; 3],
    max_x: i32,
    max_y: i32,
    radius: f32,
    color: graphics::Color
}

impl Cursor {

    // Public functions

    pub fn new(path: String, max_x: i32, max_y: i32, radius: f32, color: graphics::Color) -> Cursor {
        Cursor {
            x: 0,
            y: 0,
            f: File::open(path).unwrap(),
            buffer: [0; 3],
            max_x: max_x,
            max_y: max_y,
            radius: radius,
            color: color
        }
    }

    pub fn update(&mut self) {
        self.f.read(&mut self.buffer).unwrap();
        let sx = self.buffer[1] as i8 as i32;
        let sy = -(self.buffer[2] as i8 as i32);

        self.x += sx;
        self.y += sy;

        self.set_min_max();
    }

    pub fn print_absolute_position(&self) {
        println!("x : {}, y: {}", self.x, self.y);
    }

    // Graphics
    
    // checks if the given coordinate is at the cursor location
    pub fn change_pixel_in_cursor(&self, pixel: &mut graphics::Pixel){
        let dx = pixel.x-(self.x as u32);
        let dy = pixel.y-(self.y as u32);

        if (((dx*dx)+(dy*dy)) as f32).sqrt() < self.radius {
            pixel.color = self.color;
        }
        
    }

    // Private functions

    fn set_min_max(&mut self) {
        if self.x > self.max_x {
            self.x = self.max_x;
        }
        if self.y > self.max_y {
            self.y = self.max_y;
        }
        if self.x < 0 {
            self.x = 0;
        }
        if self.y < 0 {
            self.y = 0;
        }
    }

}