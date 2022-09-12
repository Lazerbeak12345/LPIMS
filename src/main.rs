use lpims::{
    arguments::Args,
    tokens::get_token_stream,
    parse::get_streaming_parse_tree,
    file_utils::get_file_content
};
use clap::Parser;

fn generate_output() {
    todo!("generate output");
    todo!("convert tree into data-form LLVM IR");
    todo!("collect LLVM compile flags (and other metadata)");
    todo!("run llvm on the tree")
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
