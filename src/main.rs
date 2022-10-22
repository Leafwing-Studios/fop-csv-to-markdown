use std::{fs, path::PathBuf};

use clap::Parser;
use fop_csv_to_markdown::{process_dir, process_file};

#[derive(Parser, Debug)]
struct Args {
    /// CSV file or directory to process.
    /// If given a directory, the directory is searched recursively,
    /// and the transformation is applied to all CSV files found.
    in_path: PathBuf,

    /// Index of the column to be used as the "header" for the final markdown snippet
    #[arg(short = 'h', long, default_value_t = 0)]
    header_field_index: usize,

    /// Header level (e.g. h1, h2, h3, etc.) to use for the header item
    #[arg(short = 'l', long, default_value_t = 2)]
    header_level: usize,

    /// Filename to output finished markdown file to.
    /// If not provided, outputs the result to stdout.
    /// If [`in_path`] is a directory, this is ignored
    #[arg(short, long)]
    out_file: Option<PathBuf>,

    /// Title to prepend to the top of the output
    /// If [`in_path`] is a directory, this is ignored
    #[arg(short, long)]
    title: Option<String>,
}

fn main() {
    let args = Args::parse();

    if args
        .in_path
        .metadata()
        .expect("Failure returing path metadata")
        .is_file()
    {
        let mut markdown = process_file(&args.in_path, args.header_field_index, args.header_level);

        if let Some(title) = args.title {
            markdown = format!("# {}\n\n", title) + &markdown;
        }

        if let Some(path) = args.out_file {
            fs::write(path, markdown).expect("Failure writing output file");
        } else {
            println!("{}", markdown);
        }
    } else {
        process_dir(args.in_path, args.header_field_index, args.header_level);
    }
}
