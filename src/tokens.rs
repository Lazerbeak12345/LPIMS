fn get_initial_regexes() {
    todo!("compile initial regular expressions");
    todo!("The list of named regexes is dynamic, and can be modified during the
          parse phase.");
}

/// Stream token sequence from input file
pub fn get_token_stream(file: String) {
    get_initial_regexes();
    todo!("load file `{}` into buffer to be operated upon", file);
    {
        todo!("Use current regexps to get next token");
        todo!("Tokens are granted upon request.");
        todo!("Can be told to skip.");
    }
}
