use regex::{Regex, Error as ReError};

/// A very slow regex::RegexSet that lets you change the sets on the fly
type RegexVec = Vec<Regex>;
macro_rules! regexVecUncompiled {
    ( $( $x:expr ),* ) => {
        vec![
            $(
                Regex::new($x)
            ),*
        ]
    };
}
macro_rules! regexVec {
    ( $( $x:expr ),* ) => {
        regexVecUncompiled![
            $( $x ),*
        ].into_iter().map(|reg:Result<Regex,ReError>| reg.unwrap()).collect()
    };
}

/** Returns the "default" regexes.
 *
 * TODO test that The list of named regexes is dynamic, and can be modified during the parse phase.
 */
fn get_initial_regexes() -> RegexVec {
    todo!("Fill out token list");
    regexVec![]
}

/// Stream token sequence from input file
pub fn get_token_stream(data: String) {
    get_initial_regexes();
    todo!("do something with '{}'", data);
    {
        todo!("Use current regexps to get next token");
        todo!("Tokens are granted upon request.");
        todo!("Tokens include location and path data.");
        todo!("Can be told to skip.");
    }
}
