extern crate lopdf;

use clap::{Parser, Subcommand, ArgGroup};

pub mod utils;


const DEG_MULTIPLE: i32 = 90;

#[derive(Parser, Debug)]
#[clap(name = "tpdf")]
#[clap(author = "Thomas C.")]
#[clap(version = "0.1.0")]
#[clap[about = "A tool for PDF manipulation"]]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[clap(arg_required_else_help = false)]
    /// Merges PDFs into a single file
    Merge {
        #[clap(required = true, parse(from_os_str))]
        infiles: Vec<std::path::PathBuf>,
        #[clap(required = true, parse(from_os_str))]
        outfile: std::path::PathBuf,
        #[clap(short, long)]
        compress: bool
    },
    #[clap(arg_required_else_help = false)]
    /// Splits each page of a PDF into a separate file
    Split {
        #[clap(required = true, parse(from_os_str))]
        infile: std::path::PathBuf,
        #[clap(required = true, parse(from_os_str))]
        outfile: std::path::PathBuf,
        #[clap(short, long)]
        compress: bool
    },
    #[clap(arg_required_else_help = false)]
    /// Duplicates a PDF n times and saves the duplicates into a single file
    Dupe {
        #[clap(required = true, parse(from_os_str))]
        infile: std::path::PathBuf,
        #[clap(required = true, parse(from_os_str))]
        outfile: std::path::PathBuf,
        #[clap(required = true, short, long)]
        // Number of times to duplicate
        num: u16,
        #[clap(short, long)]
        compress: bool
    },
    #[clap(arg_required_else_help = false)]
    #[clap(group(
        ArgGroup::new("rot")
            .required(false)
            .args(&["pages", "every"])
        ))]
    /// Rotate an entire document, or select pages
    Rotate {
        #[clap(required = true, parse(from_os_str))]
        infile: std::path::PathBuf,
        /// Modified inplace if not provided
        #[clap(required = false, parse(from_os_str))]
        outfile: Option<std::path::PathBuf>,
        #[clap(required=true, value_parser = degree_in_range, short, long)]
        /// Positive values are CW, negative are CCW rotation. Multipules of 90.
        degrees: i32,
        #[clap(group = "rot", short, long, multiple=true, value_parser)]
        /// List of space separated page numbers. All pages if not provided.
        pages: Option<Vec<u32>>,
        #[clap(group = "rot", short, long, value_parser)]
        every: Option<u32>
    },
    #[clap(arg_required_else_help = false)]
    #[clap(group(
        ArgGroup::new("dels")
            .required(false)
            .args(&["pages", "every"])
        ))]
    /// Delete pages from a PDF. 
    /// A list of space separated pages or --every ith page
    Delete {
        #[clap(required = true, parse(from_os_str))]
        infile: std::path::PathBuf,
        #[clap(required = false, parse(from_os_str))]
        /// Modified inplace if not provided
        outfile: Option<std::path::PathBuf>,
        #[clap(group = "dels", short, long,  multiple=true, value_parser)]
        /// List of space separated page numbers
        pages: Option<Vec<u32>>,
        #[clap(group = "dels", short, long, value_parser)]
        /// Delete every ith page
        every: Option<u32>,
        #[clap(required=false, long)]
        /// Negates the deletion operation, i.e. keep only the listed pages. 
        /// Used with --every, it will keep every ith page rather than delete it.
        negate: bool,
        #[clap(short, long)]
        compress: bool

    },
    #[clap(arg_required_else_help = false)]
    /// Reverse the order of a PDF
    Reverse {
        #[clap(required = true, parse(from_os_str))]
        infile: std::path::PathBuf,
        #[clap(required = false, parse(from_os_str))]
        /// Modified inplace if not provided
        outfile: Option<std::path::PathBuf>,
    },
    #[clap(arg_required_else_help = false)]
    #[clap(group(
        ArgGroup::new("extract")
            .required(false)
            .args(&["pages", "every"])
        ))]
    /// Extract specific pages from a PDF
    Extract {
        #[clap(required = true, parse(from_os_str))]
        infile: std::path::PathBuf,
        #[clap(required = true, parse(from_os_str))]
        /// Modified inplace if not provided
        outfile: std::path::PathBuf,
        #[clap(group = "extract", short, long,  multiple=true, value_parser)]
        /// List of space separated page numbers
        pages: Option<Vec<u32>>,
        #[clap(group = "extract", short, long, value_parser)]
        /// Delete every ith page
        every: Option<u32>,
    }
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Merge { mut infiles, outfile, compress } => {
            // TODO
            println!("Not Implemented");
        }
        Commands::Split { infile, outfile, compress} => {
            // TODO
            println!("Not Implemented");
        }
        Commands::Dupe { infile, outfile, num, compress} => {
            // TODO
            println!("Not Implemented");
        }
        Commands::Rotate { infile,
                           outfile, 
                           degrees, 
                           pages,
                           every } => {
            utils::rotate(infile, outfile, degrees, pages, every);
        },
        Commands::Delete { infile, 
                           outfile, 
                           pages, 
                           every, 
                           negate,
                           compress } => {

            utils::delete(infile, outfile, pages, every, negate, compress);
        },
        Commands::Reverse { infile, outfile } => {
            utils::reverse(infile, outfile);
        },
        Commands::Extract { infile, 
                            outfile, 
                            pages, 
                            every } => {
            utils::extract(infile, outfile, pages, every);
        }
    }    
}


fn degree_in_range(s: &str) -> Result<i32, String> {
    let degree: i32 = s
        .parse()
        .map_err(|_| format!("`{}` is not an integer", s))?;
    if degree % DEG_MULTIPLE == 0 {
        Ok(degree)
    } else {
        Err(format!(
            "degrees must be a multiple of {}",
            DEG_MULTIPLE
        ))
    }
}
