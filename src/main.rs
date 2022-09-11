use lpims::{
    arguments::Args,
    tokens::get_token_stream,
    parse::get_streaming_parse_tree
};
use clap::Parser;
use std::{path::PathBuf,fs::File};

fn generate_output() {
    todo!("generate output");
    todo!("convert tree into data-form LLVM IR");
    todo!("collect LLVM compile flags (and other metadata)");
    todo!("run llvm on the tree")
}
fn get_file(path:PathBuf) -> File {
    match File::open(&path) {
        Ok(file) => file,
        Err(msg) => panic!("{} : {}", path.display(), msg)
    }
}

fn main() {
    let args = Args::parse();
    println!("Input file {}", args.path.display());
    get_file(args.path);
    //TODO shabang to specify base-language.
    get_token_stream();
    get_streaming_parse_tree();
    generate_output();
}
