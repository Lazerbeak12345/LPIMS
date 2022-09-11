use lpims::arguments::Args;
use clap::Parser;
use std::path::PathBuf;

fn get_token_stream() {
    todo!("Stream token sequence from input file");
    todo!("The list of named regexes is dynamic, and can be modified during the
          parse phase.");
    todo!("Tokens are granted upon request.");
    todo!("Can be told to skip.");
    todo!("return something usefull");
}
fn get_streaming_parse_tree() {
    todo!("Interativly consume token sequence using a dynamic parse tree.");
    {
        todo!("If a pattern matching the `delete-macro` pattern is found, the named
              macro is deleted.");
        todo!("If a pattern matching the `macro-simple` pattern is found, the simple
              macro is added to the parse tree (named)");
        todo!("If a pattern matching the `macro-token` pattern is found, the macro is
              added to the tokenizer (this is why we stream from it, so it can
                                      change as we read from it) (named)");
        {
            todo!("If a pattern matching the `macro-complex` pattern is found, the body
                  is converted into a seperate lpims file, with the target set to
                  webassembly. When the macro matches, the webassembly is loaded,
                  and requested to parse the remainder of the file. It may exit with
                  these states:");
            todo!(" - An error");
            todo!(" - The resulting parse tree with the info on where to continue parsing from.");
        }
    }
    {
        todo!("It is provided with:");
        todo!(" - An API to fully interact with the token sequencer");
        todo!(" - An API to request that this parser parse a given token sequence");
    }
}
fn generate_output() {
    todo!("generate output");
    todo!("convert tree into data-form LLVM IR");
    todo!("collect LLVM compile flags (and other metadata)");
    todo!("run llvm on the tree")
}
fn get_file(path:PathBuf) {
    todo!("load or stream input file")
}

fn main() {
    let args = Args::parse();
    println!("Input file {}", args.path.display());
    get_file(args.path);
    // todo shabang to specify base-language.
    get_token_stream();
    get_streaming_parse_tree();
    generate_output();
}
