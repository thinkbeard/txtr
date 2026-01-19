use crate::encoder;
use image::{DynamicImage, GenericImageView, ImageError};
use std::path::Path;

pub struct Txtr {
    pub channel: encoder::EncoderFn,
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub levels: Vec<Option<usize>>,
    pub max: usize,
    pub min: usize,
    pub img: DynamicImage,
}

impl Txtr {
    pub fn new(
        file: &str,
        channel: encoder::EncoderFn,
        red: f64,
        green: f64,
        blue: f64,
    ) -> Result<Txtr, ImageError> {
        let path = Path::new(file);

        Ok(Txtr {
            channel,
            red,
            green,
            blue,
            levels: vec![],
            max: 0,
            min: usize::MAX,
            img: image::open(path)?,
        })
    }

    pub fn invert(&mut self) {
        self.img.invert();
    }

    pub fn outline(&mut self) {
        let kernel = [-1.0f32, -1.0, -1.0, -1.0, 8.0, -1.0, -1.0, -1.0, -1.0];
        self.img = self.img.filter3x3(&kernel);
    }

    pub fn resize(&mut self, width: u32, fontsize: f32) {
        let aspect_ratio = self.img.height() as f32 / self.img.width() as f32;
        let new_height = (width as f32 * aspect_ratio * fontsize) as u32;

        self.img = self.img.resize_exact(
            width,
            new_height.max(1),
            image::imageops::FilterType::Gaussian,
        );
    }

    pub fn get_level(&self, r: u8, g: u8, b: u8, a: u8) -> usize {
        (self.channel)(
            f64::from(r) * self.red,
            f64::from(g) * self.green,
            f64::from(b) * self.blue,
            f64::from(a),
        )
    }

    pub fn calc_levels(&mut self) {
        let width = self.img.width();
        let height = self.img.height();

        for y in 0..height {
            for x in 0..width {
                let pixel = self.img.get_pixel(x, y);
                let level = self.get_level(pixel[0], pixel[1], pixel[2], pixel[3]);

                if level > self.max {
                    self.max = level;
                }

                if level < self.min {
                    self.min = level;
                }

                self.levels.push(Some(level));
            }
            self.levels.push(None);
        }

        // Handle edge case where no pixels were processed
        if self.min == usize::MAX {
            self.min = 0;
        }
        if self.max == 0 && self.min == 0 {
            self.max = 255;
        }
    }

    pub fn print_by_level(&self, s: &str) {
        let chars: Vec<char> = s.chars().collect();
        let char_count = chars.len();

        // Avoid division by zero
        let range = if self.max > self.min {
            (self.max - self.min) / char_count + 1
        } else {
            1
        };

        let ascii: String = self
            .levels
            .iter()
            .map(|l| match l {
                Some(v) => {
                    let adjusted = v.saturating_sub(self.min);
                    let index = (adjusted / range).min(char_count - 1);
                    chars[index]
                }
                None => '\n',
            })
            .collect();

        println!("{}", ascii);
    }

    pub fn print_in_order(&self, s: &str, level: usize) {
        let chars: Vec<char> = s.chars().collect();
        let mut count = 0;
        let chars_len = chars.len();

        let ascii: String = self
            .levels
            .iter()
            .map(|l| match l {
                Some(v) => {
                    if count >= chars_len {
                        count = 0;
                    }

                    let c = chars[count];

                    if *v > level {
                        count += 1;
                        c
                    } else {
                        ' '
                    }
                }
                None => '\n',
            })
            .collect();

        println!("{}", ascii);
    }
}
