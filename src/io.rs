use anyhow::{ensure, Result};
use std::fs::File;
use std::io::{self, BufReader, Read, Write};
use std::path::{Path, PathBuf};

pub fn read_binary<P>(path: P) -> Result<Vec<u8>>
where
    P: AsRef<Path>,
{
    let mut file = File::open(path)?;
    let mut buf = Vec::<u8>::new();
    file.read_to_end(&mut buf)?;
    Ok(buf)
}

pub fn write_binary<P>(path: P, data: &[u8]) -> Result<()>
where
    P: AsRef<Path>,
{
    let mut file = File::create(path)?;
    let buf = BufReader::new(data)
        .bytes()
        .collect::<io::Result<Vec<u8>>>()?;
    file.write_all(&buf)?;
    file.flush()?;
    Ok(())
}

pub fn change_extention(filepath: &PathBuf, to_ext: &str) -> Result<PathBuf> {
    let mut path = std::fs::canonicalize(filepath).unwrap();

    ensure!(path.is_file(), "the path is not a file.");
    path.set_extension(to_ext);
    Ok(path)
}
