use encoder;
use image::{DynamicImage, GenericImageView};
use image;
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
    pub fn new(file: &str, channel: encoder::EncoderFn, red: f64, green: f64, blue: f64) -> Txtr {
        let path = Path::new(file);

        Txtr {
            channel: channel,
            red: red,
            green: green,
            blue: blue,
            levels: vec![],
            max: 255,
            min: 0,
            img: image::open(path).expect(&format!("Could not load image at {:?}", path)),
        }
    }

    pub fn invert(&mut self) {
        self.img.invert();
    }

    pub fn outline(&mut self) {
        let kernel = [-1.0f32, -1.0, -1.0, -1.0, 8.0, -1.0, -1.0, -1.0, -1.0];
        self.img = self.img.filter3x3(&kernel);
    }

    pub fn resize(&mut self, width: u32, fontsize: f32) {
        let new_ratio = self.img.width() as f32 / self.img.height() as f32;
        let new_height = (width as f32 * new_ratio) * fontsize;

        self.img = self.img.resize_exact(
            width,
            new_height as u32,
            image::FilterType::Gaussian,
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
        let width = self.img.width() - 1;

        self.img.clone().pixels().for_each(|p| {
            let level = self.get_level(p.2.data[0], p.2.data[1], p.2.data[2], p.2.data[3]);

            if level > self.max {
                self.max = level;
            }

            if level < self.min {
                self.min = level;
            }

            self.levels.push(Some(level));

            if p.0 == width {
                self.levels.push(None);
            }
        });
    }

    pub fn print_by_level(&mut self, s: &str) {
        let chars = s.chars().collect::<Vec<char>>();
        let range = ((self.max - self.min) / chars.len()) + 1;

        let ascii = self.levels
            .iter()
            .map(|l| match l.as_ref() {
                Some(v) => chars[v / range],
                None => '\n',
            })
            .collect::<String>();

        println!("{}", ascii);
    }

    pub fn print_in_order(&mut self, s: &str, level: usize) {
        let chars = s.chars().collect::<Vec<char>>();
        let mut count = 0;
        let charslen = chars.len();

        let ascii = self.levels
            .iter()
            .map(|l| match l.as_ref() {
                Some(v) => {
                    if count >= charslen {
                        count = 0;
                    }

                    let char = chars[count];

                    if v > &level {
                        count += 1;
                        char
                    } else {
                        ' '
                    }
                }
                None => '\n',
            })
            .collect::<String>();

        println!("{}", ascii);
    }
}
