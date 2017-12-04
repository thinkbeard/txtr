pub const USAGE: &str = "
txtr

Usage:
  txtr [options] <file>
  txtr (-h | --help)

Options:
  -h --help     Show this screen.

  -w --width=<characters>  Sets width number of character to print image. [default: 80]

  -f --fontsize=<ratio>  Ratio of heigth to width against font size. [default: 1]

  -c --char=<characters>  Sets character to use for image. Adding characters increases dither. [default: #$%{/;:,.. ]

  -p --print-in-order  Set of characters to use in text
                                        
  -l --level=<level>  Level threshold for determining when to print characters in order [default: 127]

  -e --encoder=<name>    Encoder Name [default: luma_601]

		red     - red channel only
		green   - green channel only
		blue    - blue channel only
		alpha   - alpha channel only
		luma601 - converts image to luma 601
		luma709 - converts image to luma 709

  -r --red=<percent>    Percent of red channel to be used. [default: 1.0]

  -g --green=<percent>  Percent of green channel to use [default: 1.0]

  -b --blue=<percent>   Percent of blue channel to use [default: 1.0]

  -i --invert   Invert image

  -o --outline  Outline image by applying a 3x3 kernel filter
";

#[derive(Debug, Deserialize)]
pub struct Args {
    pub arg_file: String,
    pub flag_encoder: String,
    pub flag_blue: f64,
    pub flag_green: f64,
    pub flag_red: f64,
    pub flag_fontsize: f32,
    pub flag_outline: bool,
    pub flag_invert: bool,
    pub flag_width: u32,
    pub flag_char: String,
    pub flag_print_in_order: bool,
    pub flag_level: usize,
}
