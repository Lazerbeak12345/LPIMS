use lpims::{
    arguments::Args,
    tokens::get_token_stream,
    parse::get_streaming_parse_tree
};
use clap::Parser;
use std::{path::PathBuf,fs::File,io::Read};

fn generate_output() {
    todo!("generate output");
    todo!("convert tree into data-form LLVM IR");
    todo!("collect LLVM compile flags (and other metadata)");
    todo!("run llvm on the tree")
}
fn get_file_content(path:PathBuf) -> String {
    match File::open(&path) {
        Ok(mut file) => {
            let mut buf : String = "".to_string();
            Read::read_to_string(&mut file, &mut buf).unwrap();
            buf
        }
        Err(msg) => panic!("{} : {}", path.display(), msg)
    }
}

fn main() {
    let args = Args::parse();
    println!("Input file {}", args.path.display());
    let file_string = get_file_content(args.path);
    //TODO shabang to specify base-language?
    get_token_stream(file_string);
    get_streaming_parse_tree();
    generate_output();
}
