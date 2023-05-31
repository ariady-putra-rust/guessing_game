use std::{
    env::current_dir,
    fs::{canonicalize, File, Metadata},
    io::{Error, ErrorKind, Read, Result},
    path::PathBuf,
};

pub struct FilePath {
    get_path: String,
}

impl FilePath {
    pub fn access<T: Into<String>>(file_path: T) -> Self {
        Self {
            get_path: file_path.into(),
        }
    }

    // pub fn from_string(file_path: &String) -> Self {
    //     Self {
    //         get_path: file_path.to_string(),
    //     }
    // }

    // pub fn from_str(file_path: &str) -> Self {
    //     Self {
    //         get_path: file_path.to_string(),
    //     }
    // }

    pub fn get_full_path(&self) -> Result<String> {
        Ok(canonicalize(&self.get_path)?.display().to_string())
    }

    pub fn get_relative_path(&self) -> Result<String> {
        match PathBuf::from(&self.get_full_path()?).strip_prefix(&current_dir()?) {
            Ok(p) => match p.to_str() {
                Some(s) => Ok(s.to_string()),
                None => Err(Error::new(ErrorKind::InvalidData, "&Path.to_str() error")),
            },
            Err(x) => Err(Error::new(ErrorKind::InvalidInput, x)),
        }
    }

    pub fn read_string(&self) -> Result<Text> {
        read_string(&self)
    }

    pub fn read_lines(&self) -> Result<Lines> {
        read_lines(&self)
    }

    pub fn get_metadata(&self) -> Result<Metadata> {
        get_metadata(&self)
    }
}

fn get_file(file_path: &FilePath) -> Result<File> {
    File::open(&file_path.get_path)
}

type Text = String;
pub fn read_string(file_path: &FilePath) -> Result<Text> {
    let mut buf = String::new();
    get_file(&file_path)?.read_to_string(&mut buf)?;

    return Ok(buf);
}

type Line = String;
type Lines = Vec<Line>;
pub fn read_lines(file_path: &FilePath) -> Result<Lines> {
    Ok(read_string(file_path)?
        .lines()
        .map(|line| line.to_string())
        .collect())
}

pub fn get_metadata(file_path: &FilePath) -> Result<Metadata> {
    get_file(&file_path)?.metadata()
}
