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
    ((r + r + b + g + g + g) / 6.0) as usize
}
