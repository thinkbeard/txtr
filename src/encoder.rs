pub type EncoderFn = Box<Fn(f64, f64, f64, f64) -> usize>;

pub fn select(enc: &str) -> EncoderFn {
    let enc = match enc {
        "red" => red,
        "green" => green,
        "blue" => blue,
        "alpha" => alpha,
        "luma601" => luma601,
        _ => luma709,
    };

    Box::new(enc)
}

pub fn red(_r: f64, _g: f64, _b: f64, _a: f64) -> usize {
    _r as usize
}

pub fn green(_r: f64, _g: f64, _b: f64, _a: f64) -> usize {
    _g as usize
}

pub fn blue(_r: f64, _g: f64, _b: f64, _a: f64) -> usize {
    _b as usize
}

pub fn alpha(_r: f64, _g: f64, _b: f64, _a: f64) -> usize {
    _a as usize
}

pub fn luma601(_r: f64, _g: f64, _b: f64, _a: f64) -> usize {
    (_r * 0.299 + _g * 0.587 + _b * 0.114) as usize
}

pub fn luma709(_r: f64, _g: f64, _b: f64, _a: f64) -> usize {
    ((_r + _r + _b + _g + _g + _g) / 6.0) as usize
}
