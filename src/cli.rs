use clap::Parser;

/// txtr converts images to text art
#[derive(Parser, Debug)]
#[command(name = "txtr")]
#[command(version)]
#[command(about = "Convert images to ASCII text art", long_about = None)]
pub struct Args {
    /// Image file to convert
    pub file: String,

    /// Sets width number of characters to print image
    #[arg(short, long, default_value_t = 80)]
    pub width: u32,

    /// Ratio of height to width against font size
    #[arg(short, long, default_value_t = 1.0)]
    pub fontsize: f32,

    /// Characters to use for image. Adding characters increases dither
    #[arg(short, long, default_value = "#$%{/;:,.. ")]
    pub chars: String,

    /// Print characters in sequence instead of by brightness level
    #[arg(short, long)]
    pub print_in_order: bool,

    /// Level threshold for determining when to print characters in order
    #[arg(short, long, default_value_t = 127)]
    pub level: usize,

    /// Encoder name: red, green, blue, alpha, luma601, luma709
    #[arg(short, long, default_value = "luma601")]
    pub encoder: String,

    /// Percent of red channel to be used (0.0-1.0)
    #[arg(short, long, default_value_t = 1.0)]
    pub red: f64,

    /// Percent of green channel to use (0.0-1.0)
    #[arg(short, long, default_value_t = 1.0)]
    pub green: f64,

    /// Percent of blue channel to use (0.0-1.0)
    #[arg(short, long, default_value_t = 1.0)]
    pub blue: f64,

    /// Invert image colors
    #[arg(short, long)]
    pub invert: bool,

    /// Apply 3x3 kernel edge detection filter
    #[arg(short, long)]
    pub outline: bool,
}
