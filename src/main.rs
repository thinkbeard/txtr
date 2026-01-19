mod cli;
mod encoder;
mod txtr;

use clap::Parser;

fn main() {
    let args = cli::Args::parse();

    if args.chars.len() < 2 {
        eprintln!("Please use 2 or more characters.");
        std::process::exit(1);
    }

    if args.width < 1 {
        eprintln!("Please increase width.");
        std::process::exit(1);
    }

    if args.fontsize < 0.01 {
        eprintln!("Please increase fontsize.");
        std::process::exit(1);
    }

    // Clamp channel values to valid range
    let red = args.red.clamp(0.0, 1.0);
    let green = args.green.clamp(0.0, 1.0);
    let blue = args.blue.clamp(0.0, 1.0);

    if red != args.red || green != args.green || blue != args.blue {
        eprintln!("Warning: channel values clamped to 0.0-1.0 range");
    }

    let mut art = match txtr::Txtr::new(
        &args.file,
        encoder::select(&args.encoder),
        red,
        green,
        blue,
    ) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error loading image: {}", e);
            std::process::exit(1);
        }
    };

    if args.outline {
        art.outline();
    }

    if args.invert {
        art.invert();
    }

    art.resize(args.width, args.fontsize);

    art.calc_levels();

    if args.print_in_order {
        art.print_in_order(&args.chars, args.level);
    } else {
        art.print_by_level(&args.chars);
    }
}
