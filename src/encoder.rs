pub type EncoderFn = Box<dyn Fn(f64, f64, f64, f64) -> usize>;

pub const VALID_ENCODERS: &[&str] = &["red", "green", "blue", "alpha", "luma601", "luma709"];

pub fn select(enc: &str) -> EncoderFn {
    let encoder = match enc {
        "red" => red,
        "green" => green,
        "blue" => blue,
        "alpha" => alpha,
        "luma601" => luma601,
        "luma709" => luma709,
        _ => {
            eprintln!(
                "Warning: unknown encoder '{}', using 'luma709'. Valid encoders: {}",
                enc,
                VALID_ENCODERS.join(", ")
            );
            luma709
        }
    };

    Box::new(encoder)
}

pub fn red(r: f64, _g: f64, _b: f64, _a: f64) -> usize {
    r as usize
}

pub fn green(_r: f64, g: f64, _b: f64, _a: f64) -> usize {
    g as usize
}

pub fn blue(_r: f64, _g: f64, b: f64, _a: f64) -> usize {
    b as usize
}

pub fn alpha(_r: f64, _g: f64, _b: f64, a: f64) -> usize {
    a as usize
}

pub fn luma601(r: f64, g: f64, b: f64, _a: f64) -> usize {
    (r * 0.299 + g * 0.587 + b * 0.114) as usize
}

pub fn luma709(r: f64, g: f64, b: f64, _a: f64) -> usize {
    // ITU-R BT.709 coefficients for HDTV
    (r * 0.2126 + g * 0.7152 + b * 0.0722) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    // select() tests
    #[test]
    fn select_valid_encoders() {
        // All 6 valid encoder names should work without warnings
        let encoders = ["red", "green", "blue", "alpha", "luma601", "luma709"];
        for name in encoders {
            let encoder = select(name);
            // Just verify it returns a valid function by calling it
            let _ = encoder(100.0, 100.0, 100.0, 100.0);
        }
    }

    #[test]
    fn select_unknown_encoder_falls_back() {
        // Unknown encoder should fall back to luma709
        let encoder = select("invalid");
        let result = encoder(255.0, 0.0, 0.0, 255.0);
        let expected = luma709(255.0, 0.0, 0.0, 255.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn select_empty_string() {
        // Empty string should fall back to luma709
        let encoder = select("");
        let result = encoder(255.0, 0.0, 0.0, 255.0);
        let expected = luma709(255.0, 0.0, 0.0, 255.0);
        assert_eq!(result, expected);
    }

    // luma709() tests
    #[test]
    fn luma709_black() {
        // (0,0,0,_) -> 0
        assert_eq!(luma709(0.0, 0.0, 0.0, 255.0), 0);
    }

    #[test]
    fn luma709_white() {
        // (255,255,255,_) -> 254 or 255 (floating point precision)
        // 0.2126 + 0.7152 + 0.0722 = 1.0, but 255.0 * 1.0 may truncate to 254
        let result = luma709(255.0, 255.0, 255.0, 255.0);
        assert!(result >= 254 && result <= 255);
    }

    #[test]
    fn luma709_red_only() {
        // (255,0,0,_) -> 54 (0.2126*255 = 54.213)
        assert_eq!(luma709(255.0, 0.0, 0.0, 255.0), 54);
    }

    #[test]
    fn luma709_green_only() {
        // (0,255,0,_) -> 182 (0.7152*255 = 182.376)
        assert_eq!(luma709(0.0, 255.0, 0.0, 255.0), 182);
    }

    #[test]
    fn luma709_blue_only() {
        // (0,0,255,_) -> 18 (0.0722*255 = 18.411)
        assert_eq!(luma709(0.0, 0.0, 255.0, 255.0), 18);
    }

    // luma601() tests
    #[test]
    fn luma601_coefficients() {
        // Verify coefficients: 0.299, 0.587, 0.114
        // Red only: 0.299 * 255 = 76.245 -> 76
        assert_eq!(luma601(255.0, 0.0, 0.0, 255.0), 76);
        // Green only: 0.587 * 255 = 149.685 -> 149
        assert_eq!(luma601(0.0, 255.0, 0.0, 255.0), 149);
        // Blue only: 0.114 * 255 = 29.07 -> 29
        assert_eq!(luma601(0.0, 0.0, 255.0, 255.0), 29);
        // White: 0.299*255 + 0.587*255 + 0.114*255 = 255
        assert_eq!(luma601(255.0, 255.0, 255.0, 255.0), 255);
    }

    // Channel functions tests
    #[test]
    fn red_extracts_red_channel() {
        assert_eq!(red(128.0, 64.0, 32.0, 255.0), 128);
        assert_eq!(red(0.0, 255.0, 255.0, 255.0), 0);
        assert_eq!(red(255.0, 0.0, 0.0, 0.0), 255);
    }

    #[test]
    fn green_extracts_green_channel() {
        assert_eq!(green(128.0, 64.0, 32.0, 255.0), 64);
        assert_eq!(green(255.0, 0.0, 255.0, 255.0), 0);
        assert_eq!(green(0.0, 255.0, 0.0, 0.0), 255);
    }

    #[test]
    fn blue_extracts_blue_channel() {
        assert_eq!(blue(128.0, 64.0, 32.0, 255.0), 32);
        assert_eq!(blue(255.0, 255.0, 0.0, 255.0), 0);
        assert_eq!(blue(0.0, 0.0, 255.0, 0.0), 255);
    }

    #[test]
    fn alpha_extracts_alpha_channel() {
        assert_eq!(alpha(128.0, 64.0, 32.0, 255.0), 255);
        assert_eq!(alpha(255.0, 255.0, 255.0, 0.0), 0);
        assert_eq!(alpha(0.0, 0.0, 0.0, 128.0), 128);
    }

    // VALID_ENCODERS constant test
    #[test]
    fn valid_encoders_contains_all() {
        assert_eq!(VALID_ENCODERS.len(), 6);
        assert!(VALID_ENCODERS.contains(&"red"));
        assert!(VALID_ENCODERS.contains(&"green"));
        assert!(VALID_ENCODERS.contains(&"blue"));
        assert!(VALID_ENCODERS.contains(&"alpha"));
        assert!(VALID_ENCODERS.contains(&"luma601"));
        assert!(VALID_ENCODERS.contains(&"luma709"));
    }
}
