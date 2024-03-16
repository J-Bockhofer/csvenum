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

    /// Path to the output dir/file
    #[arg(short, long)]
    outpath: Option<String>,

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

}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    // need to plug in outpath below the csv path


    let csv = args.filename_csv;

    let csv_path = PathBuf::from(&csv);
    let csv_dir = csv_path.parent().expect("Failed to get CSV file directory");
    let mut output_path = PathBuf::from(csv_dir);

    if let Some(outpath) = &args.outpath {
        output_path.push(outpath);
    }

    //let outfile = if args.outfile.is_some() {PathBuf::from(args.outfile.unwrap())} else {PathBuf::from(csv.clone())};
    let split_files = args.split_properties.unwrap_or_else(|| false);
    let gen_variant_str_fns = args.variant_str_fns.unwrap_or_else(|| true);
    let gen_impl_links = args.impl_links.unwrap_or_else(|| true);
    //let experimental_no_type_restrictions = args.experimental_no_type_restrictions.unwrap_or_else(|| false);

    // Construct the EnumOptions struct
    let options = EnumOptions {
        path_to_outfile: output_path,
        split_files,
        gen_variant_str_fns,
        gen_impl_links,
        experimental_no_type_restrictions: false,
    };
    generate_configured_enum_from_csv(Some(options), csv)?;
    
    Ok(())
}