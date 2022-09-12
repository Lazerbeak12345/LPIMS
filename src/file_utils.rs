use std::{path::PathBuf,fs::File,io::Read};
pub fn get_file_content(path:PathBuf) -> String {
    match File::open(&path) {
        Ok(mut file) => {
            let mut buf : String = "".to_string();
            Read::read_to_string(&mut file, &mut buf).unwrap();
            buf
        }
        Err(msg) => panic!("{} : {}", path.display(), msg)
    }
}
