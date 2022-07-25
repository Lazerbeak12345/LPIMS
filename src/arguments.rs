use clap::Parser;
/// Compile a lpims program
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    pub path: std::path::PathBuf
}
