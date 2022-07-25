use lpims::arguments::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();
    println!("Input file {}", args.path.display());
    todo!("Stream token sequence from args
          The list of named regexes is dynamic, and can be modified during the
          parse phase.
          Tokens are granted upon request.
          Can be told to skip.
          ");
    todo!("Interativly consume token sequence using a dynamic parse tree.
          If a pattern matching the `delete-macro` pattern is found, the named
          macro is deleted.
          If a pattern matching the `macro-simple` pattern is found, the simple
          macro is added to the parse tree (named)
          If a pattern matching the `macro-token` pattern is found, the macro is
          added to the tokenizer (this is why we stream from it, so it can
                                  change as we read from it) (named)
          If a pattern matching the `macro-complex` pattern is found, the body
          is converted into a seperate lpims file, with the target set to
          webassembly. When the macro matches, the webassembly is loaded,
          and requested to parse the remainder of the file. It may exit with
          these states:
           - An error
           - The resulting parse tree with the info on where to continue
           parsing from.
          It is provided with:
           - An API to fully interact with the token sequencer
           - An API to request that this parser parse a given token sequence
          ");
}
