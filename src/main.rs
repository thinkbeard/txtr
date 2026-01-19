mod cli;
mod encoder;
mod txtr;

use clap::Parser;

// Compile-time check that cli.rs default matches txtr::DEFAULT_CHARS
// (cli.rs uses literal "#$%{/;:,.. " which must match this constant)
const _: () = {
    const CLI_DEFAULT: &str = "#$%{/;:,.. ";
    let a = txtr::DEFAULT_CHARS.as_bytes();
    let b = CLI_DEFAULT.as_bytes();
    assert!(a.len() == b.len(), "DEFAULT_CHARS length mismatch");
    let mut i = 0;
    while i < a.len() {
        assert!(a[i] == b[i], "DEFAULT_CHARS content mismatch");
        i += 1;
    }
};

fn main() {
    let args = cli::Args::parse();

    // Determine character set: use --chars if explicitly provided (non-default), otherwise use ramp preset
    let chars = if args.chars != txtr::DEFAULT_CHARS {
        args.chars.clone()
    } else {
        txtr::get_ramp(&args.ramp).to_string()
    };

    let char_count = chars.chars().count();
    if char_count < 2 {
        eprintln!("Error: Please use 2 or more characters (got {}).", char_count);
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

    // Validate channel values are not NaN
    if args.red.is_nan() || args.green.is_nan() || args.blue.is_nan() {
        eprintln!("Error: channel values cannot be NaN");
        std::process::exit(1);
    }

    // Clamp channel values to valid range
    let red = args.red.clamp(0.0, 1.0);
    let green = args.green.clamp(0.0, 1.0);
    let blue = args.blue.clamp(0.0, 1.0);

    if red != args.red || green != args.green || blue != args.blue {
        eprintln!("Warning: channel values clamped to 0.0-1.0 range");
    }

    // Warn about incompatible flag combinations
    if args.blocks {
        let mut ignored = Vec::new();
        if args.chars != txtr::DEFAULT_CHARS {
            ignored.push("--chars");
        }
        if args.ramp != "standard" {
            ignored.push("--ramp");
        }
        if args.print_in_order {
            ignored.push("--print-in-order");
        }
        if args.dither {
            ignored.push("--dither");
        }
        if !ignored.is_empty() {
            eprintln!(
                "Warning: --blocks mode ignores: {}",
                ignored.join(", ")
            );
        }
    }

    // --blocks implies color mode
    let color_enabled = args.color || args.blocks;

    let mut art = match txtr::Txtr::new(
        &args.file,
        encoder::select(&args.encoder),
        red,
        green,
        blue,
        color_enabled,
    ) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error loading image: {}", e);
            std::process::exit(1);
        }
    };

    // Validate image dimensions
    if art.img.width() == 0 || art.img.height() == 0 {
        eprintln!("Error: Image has zero width or height");
        std::process::exit(1);
    }

    if args.outline {
        art.outline();
    }

    if args.invert {
        art.invert();
    }

    // Prevent memory exhaustion from extremely large resize targets
    const MAX_PIXELS: u64 = 10_000_000; // 10 megapixels
    let aspect_ratio = art.img.height() as f64 / art.img.width() as f64;
    let new_height = (args.width as f64 * aspect_ratio * args.fontsize as f64) as u64;
    let pixels = args.width as u64 * new_height.max(1);
    if pixels > MAX_PIXELS {
        eprintln!(
            "Error: Requested dimensions too large ({} pixels, max {})",
            pixels, MAX_PIXELS
        );
        std::process::exit(1);
    }

    art.resize(args.width, args.fontsize);

    art.calc_levels(args.dither, char_count);

    if args.blocks {
        art.print_blocks();
    } else if args.print_in_order {
        art.print_in_order(&chars, args.level);
    } else {
        art.print_by_level(&chars);
    }
}
