use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

fn read_file(path: PathBuf) -> Result<Vec<u8>, std::io::Error> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = Vec::new();
    buf_reader.read_to_end(&mut contents)?;
    Ok(contents)
}
