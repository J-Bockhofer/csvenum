use clap::Parser;
use std::path::PathBuf;
// Required input
// 1. csv file
//use std::env::current_dir;
// User options:
// 1. path to file and file name
// 2. generate variant 
use csvenum::{generate_configured_enum_from_csv, EnumOptions};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    /// Filename of the CSV file (required)
    filename_csv: String,

    /// Path to the output file
    #[arg(short, long)]
    outfile: Option<String>,

    /// Whether to split property declarations into separate files, defaults to: false
    #[arg(short, long)]
    split_properties: Option<bool>,

    /// Generate variant as & from str fns , defaults to: true
    #[arg(short, long)]
    variant_str_fns: Option<bool>,

    /// Pure conversion functions only or also impl links to them, defaults to: true
    #[arg(short, long)]
    impl_links: Option<bool>,

/*     /// Unlock const type restrictions on input table
    #[arg(long)]
    experimental_no_type_restrictions: Option<bool>, */

    /// Delimiter for nested values, defaults to: '$'
    #[arg(short,long)]
    delimiter_for_nested_values: Option<char>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let csv = args.filename_csv;
    let outfile = if args.outfile.is_some() {PathBuf::from(args.outfile.unwrap())} else {PathBuf::from(csv.clone())};
    let split_files = args.split_properties.unwrap_or_else(|| false);
    let gen_variant_str_fns = args.variant_str_fns.unwrap_or_else(|| true);
    let gen_impl_links = args.impl_links.unwrap_or_else(|| true);
    let multival_split_symbol = args.delimiter_for_nested_values.unwrap_or_else(|| '$').to_string(); // Default to "$"
    //let experimental_no_type_restrictions = args.experimental_no_type_restrictions.unwrap_or_else(|| false);

    // Construct the EnumOptions struct
    let options = EnumOptions {
        path_to_outfile: outfile,
        split_files,
        gen_variant_str_fns,
        gen_impl_links,
        experimental_no_type_restrictions: false,
        multival_split_symbol, 
    };
    generate_configured_enum_from_csv(Some(options), csv)?;
    
    Ok(())
}