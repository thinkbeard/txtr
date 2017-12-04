extern crate docopt;
extern crate image;
extern crate imageproc;

#[macro_use]
extern crate serde_derive;

mod txtr;
mod cli;
mod encoder;

use docopt::Docopt;

fn main() {
    let args: cli::Args = Docopt::new(cli::USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.flag_char.len() < 2 {
        println!("Please use 2 or more characters.");
        return;
    }

    if args.flag_width < 1 {
        println!("Please increase width.");
        return;
    }

    if args.flag_fontsize < 0.01 {
        println!("Please increase fontsize.");
        return;
    }

    let mut art = txtr::Txtr::new(
        &args.arg_file,
        encoder::select(&args.flag_encoder),
        args.flag_red,
        args.flag_green,
        args.flag_blue,
    );

    if args.flag_outline {
        art.outline();
    }

    if args.flag_invert {
        art.invert();
    }

    art.resize(args.flag_width, args.flag_fontsize);

    art.calc_levels();

    if args.flag_print_in_order {
        art.print_in_order(&args.flag_char, args.flag_level);
    } else {
        art.print_by_level(&args.flag_char);
    }
}
