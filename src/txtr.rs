use crate::encoder;
use image::{DynamicImage, GenericImageView, ImageError};
use std::path::Path;

// Character ramp presets
pub const RAMP_STANDARD: &str = " .:-=+*#%@";
pub const RAMP_DENSE: &str = " .',:;clodxkO0KXNWM";
pub const RAMP_BLOCKS: &str = " ░▒▓█";
pub const RAMP_SIMPLE: &str = " .oO@";

/// Default character set used when no --chars or --ramp is specified
pub const DEFAULT_CHARS: &str = "#$%{/;:,.. ";

pub const VALID_RAMPS: &[&str] = &["standard", "dense", "blocks", "simple"];

pub fn get_ramp(name: &str) -> &'static str {
    match name {
        "standard" => RAMP_STANDARD,
        "dense" => RAMP_DENSE,
        "blocks" => RAMP_BLOCKS,
        "simple" => RAMP_SIMPLE,
        _ => {
            eprintln!(
                "Warning: unknown ramp '{}', using 'standard'. Valid ramps: {}",
                name,
                VALID_RAMPS.join(", ")
            );
            RAMP_STANDARD
        }
    }
}

#[derive(Clone, Copy)]
pub struct PixelData {
    pub level: usize,
    pub rgb: Option<(u8, u8, u8)>,
}

pub struct Txtr {
    pub channel: encoder::EncoderFn,
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub pixels: Vec<Option<PixelData>>,
    pub max: usize,
    pub min: usize,
    pub img: DynamicImage,
    pub color_enabled: bool,
    pub width: usize,
    pub height: usize,
}

const ANSI_RESET: &str = "\x1b[0m";
const UPPER_HALF_BLOCK: char = '▀';

fn format_color(r: u8, g: u8, b: u8) -> String {
    format!("\x1b[38;2;{};{};{}m", r, g, b)
}

fn format_bg_color(r: u8, g: u8, b: u8) -> String {
    format!("\x1b[48;2;{};{};{}m", r, g, b)
}

impl Txtr {
    pub fn new(
        file: &str,
        channel: encoder::EncoderFn,
        red: f64,
        green: f64,
        blue: f64,
        color_enabled: bool,
    ) -> Result<Txtr, ImageError> {
        let path = Path::new(file);

        Ok(Txtr {
            channel,
            red,
            green,
            blue,
            pixels: vec![],
            max: 0,
            min: usize::MAX,
            img: image::open(path)?,
            color_enabled,
            width: 0,
            height: 0,
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
        let new_height = (width as f32 * aspect_ratio * fontsize)
            .min(u32::MAX as f32) as u32;

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

    pub fn calc_levels(&mut self, dither: bool, char_count: usize) {
        let width = self.img.width() as usize;
        let height = self.img.height() as usize;
        self.width = width;
        self.height = height;

        // Check for overflow in buffer allocation
        let pixel_count = match width.checked_mul(height) {
            Some(count) => count,
            None => {
                eprintln!("Error: Image dimensions too large (overflow)");
                std::process::exit(1);
            }
        };

        // First pass: collect raw levels to find min/max
        let mut raw_levels: Vec<f64> = Vec::with_capacity(pixel_count);
        let mut colors: Vec<Option<(u8, u8, u8)>> = Vec::with_capacity(pixel_count);

        for y in 0..height {
            for x in 0..width {
                let pixel = self.img.get_pixel(x as u32, y as u32);
                let level = self.get_level(pixel[0], pixel[1], pixel[2], pixel[3]);

                if level > self.max {
                    self.max = level;
                }
                if level < self.min {
                    self.min = level;
                }

                raw_levels.push(level as f64);
                colors.push(if self.color_enabled {
                    Some((pixel[0], pixel[1], pixel[2]))
                } else {
                    None
                });
            }
        }

        // Handle edge case where no pixels were processed
        if self.min == usize::MAX {
            self.min = 0;
        }
        if self.max == 0 && self.min == 0 {
            self.max = 255;
        }

        // Apply Floyd-Steinberg dithering if enabled
        // Skip dithering when range is 0 (uniform color image) to avoid division by zero
        if dither && char_count > 1 && self.max > self.min {
            let range = (self.max - self.min) as f64;
            let levels = char_count as f64;

            for y in 0..height {
                for x in 0..width {
                    let idx = y * width + x;
                    let old_val = raw_levels[idx];

                    // Quantize to one of char_count levels
                    let normalized = (old_val - self.min as f64) / range;
                    let quantized_idx = (normalized * (levels - 1.0)).round();
                    let new_val = self.min as f64 + (quantized_idx / (levels - 1.0)) * range;

                    raw_levels[idx] = new_val;
                    let error = old_val - new_val;

                    // Distribute error to neighbors (Floyd-Steinberg pattern)
                    if x + 1 < width {
                        raw_levels[idx + 1] += error * 7.0 / 16.0;
                    }
                    if y + 1 < height {
                        if x > 0 {
                            raw_levels[(y + 1) * width + (x - 1)] += error * 3.0 / 16.0;
                        }
                        raw_levels[(y + 1) * width + x] += error * 5.0 / 16.0;
                        if x + 1 < width {
                            raw_levels[(y + 1) * width + (x + 1)] += error * 1.0 / 16.0;
                        }
                    }
                }
            }
        }

        // Build pixel data with newline markers
        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                let level = raw_levels[idx].round().clamp(0.0, 255.0) as usize;
                self.pixels.push(Some(PixelData {
                    level,
                    rgb: colors[idx],
                }));
            }
            self.pixels.push(None); // newline marker
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

        // Pre-allocate output buffer: ~20 bytes per pixel with color, ~2 without
        let bytes_per_pixel = if self.color_enabled { 25 } else { 2 };
        let estimated_size = self.width * self.height * bytes_per_pixel;
        let mut output = String::with_capacity(estimated_size);
        for pixel in &self.pixels {
            match pixel {
                Some(data) => {
                    let adjusted = data.level.saturating_sub(self.min);
                    let index = (adjusted / range).min(char_count - 1);
                    let c = chars[index];

                    if let Some((r, g, b)) = data.rgb {
                        output.push_str(&format_color(r, g, b));
                        output.push(c);
                    } else {
                        output.push(c);
                    }
                }
                None => {
                    if self.color_enabled {
                        output.push_str(ANSI_RESET);
                    }
                    output.push('\n');
                }
            }
        }
        if self.color_enabled {
            output.push_str(ANSI_RESET);
        }

        print!("{}", output);
    }

    pub fn print_in_order(&self, s: &str, level: usize) {
        let chars: Vec<char> = s.chars().collect();
        let mut count = 0;
        let chars_len = chars.len();

        // Pre-allocate output buffer: ~20 bytes per pixel with color, ~2 without
        let bytes_per_pixel = if self.color_enabled { 25 } else { 2 };
        let estimated_size = self.width * self.height * bytes_per_pixel;
        let mut output = String::with_capacity(estimated_size);
        for pixel in &self.pixels {
            match pixel {
                Some(data) => {
                    if count >= chars_len {
                        count = 0;
                    }

                    let c = chars[count];

                    if data.level > level {
                        count += 1;
                        if let Some((r, g, b)) = data.rgb {
                            output.push_str(&format_color(r, g, b));
                            output.push(c);
                        } else {
                            output.push(c);
                        }
                    } else {
                        output.push(' ');
                    }
                }
                None => {
                    if self.color_enabled {
                        output.push_str(ANSI_RESET);
                    }
                    output.push('\n');
                }
            }
        }
        if self.color_enabled {
            output.push_str(ANSI_RESET);
        }

        print!("{}", output);
    }

    /// Print using Unicode half-block characters for 2x vertical resolution.
    /// Each character cell represents 2 vertical pixels using foreground/background colors.
    /// Uses self.pixels to respect dithering and other processing.
    pub fn print_blocks(&self) {
        let width = self.width;
        let height = self.height;
        // Pre-allocate: ~25 bytes per cell (fg + bg escape codes + char)
        let estimated_size = width * height.div_ceil(2) * 30;
        let mut output = String::with_capacity(estimated_size);

        // Row stride in pixels array is width + 1 (for the None newline marker)
        let row_stride = width + 1;

        // Process 2 rows at a time
        let mut y = 0usize;
        while y < height {
            for x in 0..width {
                let top_idx = y * row_stride + x;
                let (top_r, top_g, top_b) = if let Some(Some(data)) = self.pixels.get(top_idx) {
                    data.rgb.unwrap_or((0, 0, 0))
                } else {
                    (0, 0, 0)
                };

                // Bottom pixel: use next row if available, otherwise same as top
                let (bot_r, bot_g, bot_b) = if y + 1 < height {
                    let bot_idx = (y + 1) * row_stride + x;
                    if let Some(Some(data)) = self.pixels.get(bot_idx) {
                        data.rgb.unwrap_or((0, 0, 0))
                    } else {
                        (top_r, top_g, top_b)
                    }
                } else {
                    (top_r, top_g, top_b)
                };

                // BUG C2 fix: Only output ANSI codes when color is enabled
                if self.color_enabled {
                    // Top pixel = foreground, Bottom pixel = background
                    output.push_str(&format_color(top_r, top_g, top_b));
                    output.push_str(&format_bg_color(bot_r, bot_g, bot_b));
                }
                output.push(UPPER_HALF_BLOCK);
            }
            if self.color_enabled {
                output.push_str(ANSI_RESET);
            }
            output.push('\n');
            y += 2;
        }

        print!("{}", output);
    }

    /// Generate output to a String instead of printing (for testing)
    #[cfg(test)]
    pub fn render_by_level(&self, s: &str) -> String {
        let chars: Vec<char> = s.chars().collect();
        let char_count = chars.len();

        let range = if self.max > self.min {
            (self.max - self.min) / char_count + 1
        } else {
            1
        };

        let bytes_per_pixel = if self.color_enabled { 25 } else { 2 };
        let estimated_size = self.width * self.height * bytes_per_pixel;
        let mut output = String::with_capacity(estimated_size);
        for pixel in &self.pixels {
            match pixel {
                Some(data) => {
                    let adjusted = data.level.saturating_sub(self.min);
                    let index = (adjusted / range).min(char_count - 1);
                    let c = chars[index];

                    if let Some((r, g, b)) = data.rgb {
                        output.push_str(&format_color(r, g, b));
                        output.push(c);
                    } else {
                        output.push(c);
                    }
                }
                None => {
                    if self.color_enabled {
                        output.push_str(ANSI_RESET);
                    }
                    output.push('\n');
                }
            }
        }
        if self.color_enabled {
            output.push_str(ANSI_RESET);
        }
        output
    }

    /// Generate output to a String instead of printing (for testing)
    #[cfg(test)]
    pub fn render_in_order(&self, s: &str, level: usize) -> String {
        let chars: Vec<char> = s.chars().collect();
        let mut count = 0;
        let chars_len = chars.len();

        let bytes_per_pixel = if self.color_enabled { 25 } else { 2 };
        let estimated_size = self.width * self.height * bytes_per_pixel;
        let mut output = String::with_capacity(estimated_size);
        for pixel in &self.pixels {
            match pixel {
                Some(data) => {
                    if count >= chars_len {
                        count = 0;
                    }

                    let c = chars[count];

                    if data.level > level {
                        count += 1;
                        if let Some((r, g, b)) = data.rgb {
                            output.push_str(&format_color(r, g, b));
                            output.push(c);
                        } else {
                            output.push(c);
                        }
                    } else {
                        output.push(' ');
                    }
                }
                None => {
                    if self.color_enabled {
                        output.push_str(ANSI_RESET);
                    }
                    output.push('\n');
                }
            }
        }
        if self.color_enabled {
            output.push_str(ANSI_RESET);
        }
        output
    }

    /// Generate output to a String instead of printing (for testing)
    #[cfg(test)]
    pub fn render_blocks(&self) -> String {
        let width = self.width;
        let height = self.height;
        let estimated_size = width * height.div_ceil(2) * 30;
        let mut output = String::with_capacity(estimated_size);

        let row_stride = width + 1;

        let mut y = 0usize;
        while y < height {
            for x in 0..width {
                let top_idx = y * row_stride + x;
                let (top_r, top_g, top_b) = if let Some(Some(data)) = self.pixels.get(top_idx) {
                    data.rgb.unwrap_or((0, 0, 0))
                } else {
                    (0, 0, 0)
                };

                let (bot_r, bot_g, bot_b) = if y + 1 < height {
                    let bot_idx = (y + 1) * row_stride + x;
                    if let Some(Some(data)) = self.pixels.get(bot_idx) {
                        data.rgb.unwrap_or((0, 0, 0))
                    } else {
                        (top_r, top_g, top_b)
                    }
                } else {
                    (top_r, top_g, top_b)
                };

                if self.color_enabled {
                    output.push_str(&format_color(top_r, top_g, top_b));
                    output.push_str(&format_bg_color(bot_r, bot_g, bot_b));
                }
                output.push(UPPER_HALF_BLOCK);
            }
            if self.color_enabled {
                output.push_str(ANSI_RESET);
            }
            output.push('\n');
            y += 2;
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{DynamicImage, RgbaImage};

    // Helper to create a Txtr with a test image (bypassing file loading)
    fn create_test_txtr(width: u32, height: u32, pixel_value: u8, color_enabled: bool) -> Txtr {
        let mut img = RgbaImage::new(width, height);
        for pixel in img.pixels_mut() {
            *pixel = image::Rgba([pixel_value, pixel_value, pixel_value, 255]);
        }
        Txtr {
            channel: encoder::select("luma709"),
            red: 1.0,
            green: 1.0,
            blue: 1.0,
            pixels: vec![],
            max: 0,
            min: usize::MAX,
            img: DynamicImage::ImageRgba8(img),
            color_enabled,
            width: 0,
            height: 0,
        }
    }

    // Helper to create a Txtr with custom RGB pixels
    fn create_test_txtr_rgb(
        width: u32,
        height: u32,
        pixels: &[(u8, u8, u8)],
        color_enabled: bool,
    ) -> Txtr {
        let mut img = RgbaImage::new(width, height);
        for (i, pixel) in img.pixels_mut().enumerate() {
            let (r, g, b) = pixels.get(i).copied().unwrap_or((0, 0, 0));
            *pixel = image::Rgba([r, g, b, 255]);
        }
        Txtr {
            channel: encoder::select("luma709"),
            red: 1.0,
            green: 1.0,
            blue: 1.0,
            pixels: vec![],
            max: 0,
            min: usize::MAX,
            img: DynamicImage::ImageRgba8(img),
            color_enabled,
            width: 0,
            height: 0,
        }
    }

    // get_ramp() tests
    #[test]
    fn get_ramp_valid_names() {
        assert_eq!(get_ramp("standard"), RAMP_STANDARD);
        assert_eq!(get_ramp("dense"), RAMP_DENSE);
        assert_eq!(get_ramp("blocks"), RAMP_BLOCKS);
        assert_eq!(get_ramp("simple"), RAMP_SIMPLE);
    }

    #[test]
    fn get_ramp_unknown_falls_back() {
        // Unknown ramp should fall back to standard
        assert_eq!(get_ramp("typo"), RAMP_STANDARD);
        assert_eq!(get_ramp(""), RAMP_STANDARD);
        assert_eq!(get_ramp("STANDARD"), RAMP_STANDARD); // case sensitive!
    }

    #[test]
    fn valid_ramps_contains_all() {
        assert_eq!(VALID_RAMPS.len(), 4);
        assert!(VALID_RAMPS.contains(&"standard"));
        assert!(VALID_RAMPS.contains(&"dense"));
        assert!(VALID_RAMPS.contains(&"blocks"));
        assert!(VALID_RAMPS.contains(&"simple"));
    }

    // get_level() tests
    #[test]
    fn get_level_with_channel_weights() {
        let mut txtr = create_test_txtr(1, 1, 128, false);
        txtr.red = 0.5;
        txtr.green = 0.5;
        txtr.blue = 0.5;
        // With weights of 0.5, channels are halved before encoding
        let level = txtr.get_level(200, 100, 50, 255);
        // Expected: luma709(100, 50, 25, 255)
        // = 100*0.2126 + 50*0.7152 + 25*0.0722
        // = 21.26 + 35.76 + 1.805 = 58.825 -> 58
        assert_eq!(level, 58);
    }

    // calc_levels() tests
    #[test]
    fn calc_levels_uniform_image() {
        let mut txtr = create_test_txtr(3, 3, 128, false);
        txtr.calc_levels(false, 10);
        // All same pixel -> min == max
        assert_eq!(txtr.min, txtr.max);
        assert_eq!(txtr.width, 3);
        assert_eq!(txtr.height, 3);
    }

    #[test]
    fn calc_levels_dithering_disabled_when_range_zero() {
        let mut txtr = create_test_txtr(3, 3, 128, false);
        // This should not panic even with dither=true when all pixels are same
        txtr.calc_levels(true, 10);
        assert_eq!(txtr.min, txtr.max);
    }

    #[test]
    fn calc_levels_pixel_count_correct() {
        let mut txtr = create_test_txtr(4, 3, 128, false);
        txtr.calc_levels(false, 10);
        // pixels should have width*height + height (for newline markers)
        // = 4*3 + 3 = 15
        assert_eq!(txtr.pixels.len(), 15);
        // Count newline markers (None values)
        let newline_count = txtr.pixels.iter().filter(|p| p.is_none()).count();
        assert_eq!(newline_count, 3);
    }

    #[test]
    fn calc_levels_min_max_computed() {
        // Create image with varying brightness
        let pixels = vec![
            (0, 0, 0),       // black
            (128, 128, 128), // gray
            (255, 255, 255), // white
            (64, 64, 64),    // dark gray
        ];
        let mut txtr = create_test_txtr_rgb(2, 2, &pixels, false);
        txtr.calc_levels(false, 10);
        assert_eq!(txtr.min, 0);
        // Max is 254 or 255 due to floating point precision in luma709
        assert!(txtr.max >= 254 && txtr.max <= 255);
    }

    // print_by_level() tests
    #[test]
    fn print_by_level_maps_to_correct_chars() {
        let pixels = vec![
            (0, 0, 0),       // black -> first char
            (255, 255, 255), // white -> last char
        ];
        let mut txtr = create_test_txtr_rgb(2, 1, &pixels, false);
        txtr.calc_levels(false, 2);
        let output = txtr.render_by_level("ab");
        assert_eq!(output, "ab\n");
    }

    #[test]
    fn print_by_level_uniform_image_single_char() {
        let mut txtr = create_test_txtr(3, 1, 128, false);
        txtr.calc_levels(false, 5);
        let output = txtr.render_by_level("abcde");
        // Uniform image with min==max, all pixels map to same char
        // Due to the range calculation, this will map to the first character
        assert!(output.chars().filter(|c| *c != '\n').all(|c| c == output.chars().next().unwrap()));
    }

    // print_in_order() tests
    #[test]
    fn print_in_order_cycles_chars() {
        let pixels = vec![
            (255, 255, 255), // bright - will print
            (255, 255, 255), // bright
            (255, 255, 255), // bright
        ];
        let mut txtr = create_test_txtr_rgb(3, 1, &pixels, false);
        txtr.calc_levels(false, 2);
        let output = txtr.render_in_order("ab", 0);
        // Cycles through 'a', 'b', 'a'
        assert_eq!(output, "aba\n");
    }

    #[test]
    fn print_in_order_respects_threshold() {
        let pixels = vec![
            (255, 255, 255), // level 255 - above threshold
            (0, 0, 0),       // level 0 - below threshold
            (255, 255, 255), // level 255 - above threshold
        ];
        let mut txtr = create_test_txtr_rgb(3, 1, &pixels, false);
        txtr.calc_levels(false, 2);
        let output = txtr.render_in_order("ab", 100);
        // First and third pixel print, second is below threshold (space)
        assert_eq!(output, "a b\n");
    }

    // print_blocks() tests
    #[test]
    fn print_blocks_odd_height() {
        let pixels = vec![
            (255, 0, 0), // row 0
            (0, 255, 0), // row 1
            (0, 0, 255), // row 2 (odd row)
        ];
        let mut txtr = create_test_txtr_rgb(1, 3, &pixels, true);
        txtr.calc_levels(false, 10);
        let output = txtr.render_blocks();
        // Should have 2 rows of output (rows 0-1 combined, row 2 uses itself for bottom)
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines.len(), 2);
    }

    #[test]
    fn print_blocks_1x1_image() {
        let mut txtr = create_test_txtr(1, 1, 128, true);
        txtr.calc_levels(false, 10);
        let output = txtr.render_blocks();
        // 1x1 image should produce single block character
        assert!(output.contains(UPPER_HALF_BLOCK));
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines.len(), 1);
    }

    #[test]
    fn print_blocks_respects_color_enabled() {
        let mut txtr = create_test_txtr(2, 2, 128, false); // color DISABLED
        txtr.calc_levels(false, 10);
        let output = txtr.render_blocks();
        // Should NOT contain ANSI escape codes when color is disabled
        assert!(!output.contains("\x1b["));
        // Should still contain block characters
        assert!(output.contains(UPPER_HALF_BLOCK));
    }

    #[test]
    fn print_blocks_with_color_enabled() {
        let mut txtr = create_test_txtr(2, 2, 128, true); // color ENABLED
        txtr.calc_levels(false, 10);
        let output = txtr.render_blocks();
        // Should contain ANSI escape codes when color is enabled
        assert!(output.contains("\x1b["));
        assert!(output.contains(UPPER_HALF_BLOCK));
    }

    // Color output tests
    #[test]
    fn print_by_level_no_ansi_when_color_disabled() {
        let mut txtr = create_test_txtr(2, 2, 128, false);
        txtr.calc_levels(false, 10);
        let output = txtr.render_by_level("ab");
        assert!(!output.contains("\x1b["));
    }

    #[test]
    fn print_by_level_ansi_when_color_enabled() {
        let mut txtr = create_test_txtr(2, 2, 128, true);
        txtr.calc_levels(false, 10);
        let output = txtr.render_by_level("ab");
        assert!(output.contains("\x1b["));
    }

    // Ramp character tests
    #[test]
    fn ramp_presets_have_correct_format() {
        // All ramps should start with space (for darkest pixels)
        assert!(RAMP_STANDARD.starts_with(' '));
        assert!(RAMP_DENSE.starts_with(' '));
        assert!(RAMP_BLOCKS.starts_with(' '));
        assert!(RAMP_SIMPLE.starts_with(' '));

        // All ramps should have at least 2 characters
        assert!(RAMP_STANDARD.chars().count() >= 2);
        assert!(RAMP_DENSE.chars().count() >= 2);
        assert!(RAMP_BLOCKS.chars().count() >= 2);
        assert!(RAMP_SIMPLE.chars().count() >= 2);
    }

    // DEFAULT_CHARS test
    #[test]
    fn default_chars_is_valid() {
        assert!(DEFAULT_CHARS.chars().count() >= 2);
    }
}
