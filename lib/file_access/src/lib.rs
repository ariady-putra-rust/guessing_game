use std::{
    env::current_dir,
    fs::{self, canonicalize, File, Metadata},
    io::{Error, ErrorKind, Read, Result},
    path::PathBuf,
};
use traits::to_vec_string::*;

mod traits;

pub struct FilePath {
    get_path: String,
}

impl FilePath {
    pub fn access<T: AsRef<str>>(file_path: &T) -> Self {
        Self {
            get_path: file_path.as_ref().to_string(),
        }
    }

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
        read_string(self)
    }

    pub fn read_lines(&self) -> Result<Lines> {
        read_lines(self)
    }

    pub fn write_string(&self, text: &Text) -> Result<()> {
        write_string(self, text)
    }

    pub fn write_lines(&self, lines: &Lines) -> Result<()> {
        write_lines(self, lines)
    }

    pub fn append_string(&self, text: &Text) -> Result<()> {
        append_string(self, text)
    }

    pub fn append_lines(&self, lines: &Lines) -> Result<()> {
        append_lines(self, lines)
    }

    pub fn delete(&self) -> Result<()> {
        delete(self)
    }

    pub fn copy_to(&self, to: &impl AsRef<str>) -> Result<()> {
        copy(self, to)
    }

    pub fn rename_to(&self, to: &impl AsRef<str>) -> Result<()> {
        rename(self, to)
    }

    pub fn get_metadata(&self) -> Result<Metadata> {
        get_metadata(self)
    }
}

impl AsRef<str> for FilePath {
    fn as_ref(&self) -> &str {
        self.get_path.as_str()
    }
}

fn get_file(file_path: &impl AsRef<str>) -> Result<File> {
    File::open(file_path.as_ref())
}

type Path = PathBuf;
fn path_of(file_path: &impl AsRef<str>) -> Path {
    PathBuf::from(file_path.as_ref())
}

fn mk_file(file_path: &impl AsRef<str>) -> Result<File> {
    if let Some(path) = path_of(file_path).parent() {
        fs::create_dir_all(path)?;
    }
    return File::create(file_path.as_ref());
}

type Text = String;
pub fn read_string<Path: AsRef<str>>(file_path: &Path) -> Result<Text> {
    let mut buf = String::new();
    get_file(file_path)?.read_to_string(&mut buf)?;

    return Ok(buf);
}

type Line = String;
type Lines = Vec<Line>;
pub fn read_lines<Path: AsRef<str>>(file_path: &Path) -> Result<Lines> {
    Ok(read_string(file_path)?
        .lines()
        .map(ToString::to_string)
        .collect())
}

pub fn write_string<Path: AsRef<str>, Text: AsRef<str>>(
    file_path: &Path,
    text: &Text,
) -> Result<()> {
    if !path_of(file_path).exists() {
        mk_file(file_path)?;
    }
    return fs::write(path_of(file_path), text.as_ref());
}

pub fn write_lines<Path: AsRef<str>, Line: AsRef<str>>(
    file_path: &Path,
    lines: &Vec<Line>,
) -> Result<()> {
    write_string(file_path, &lines.to_vec_string().join("\n"))
}

pub fn append_string<Path: AsRef<str>, Text: AsRef<str>>(
    file_path: &Path,
    text: &Text,
) -> Result<()> {
    write_string(
        file_path,
        &format!("{}{}", read_string(file_path)?, text.as_ref()),
    )
}

pub fn append_lines<Path: AsRef<str>, Line: AsRef<str>>(
    file_path: &Path,
    lines: &Vec<Line>,
) -> Result<()> {
    let mut file = read_lines(file_path)?;
    file.extend_from_slice(&lines.to_vec_string());

    return write_lines(file_path, &file);
}

pub fn delete<Path: AsRef<str>>(file_path: &Path) -> Result<()> {
    let path = path_of(file_path);

    if path.is_file() {
        return fs::remove_file(path);
    }

    if path.is_dir() {
        return fs::remove_dir_all(path);
    }

    return Err(Error::new(ErrorKind::InvalidInput, file_path.as_ref()));
}

pub fn copy<From: AsRef<str>, To: AsRef<str>>(from: &From, to: &To) -> Result<()> {
    write_string(to, &read_string(from)?)
}

pub fn rename<From: AsRef<str>, To: AsRef<str>>(from: &From, to: &To) -> Result<()> {
    copy(from, to)?;

    return delete(from);
}

pub fn get_metadata<Path: AsRef<str>>(file_path: &Path) -> Result<Metadata> {
    get_file(file_path)?.metadata()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Result;

    // cargo test -- --show-output --test-threads=1
    // cargo test <TESTNAME> --show-output

    #[test]
    fn read_string() -> Result<()> {
        Ok({
            // Arrange
            let file = FilePath::access(&"Cargo.toml");

            // Action
            let text = file.read_string()?;
            println!("{text}");

            // Assert
            assert_ne!(text.len(), 0);
        })
    }

    #[test]
    fn read_lines() -> Result<()> {
        Ok({
            // Arrange
            let file = FilePath::access(&"Cargo.toml");

            // Action
            let lines = file.read_lines()?;
            for line in &lines {
                println!("{line}");
            }

            // Assert
            assert_ne!(lines.len(), 0);
        })
    }

    #[test]
    fn write_string() -> Result<()> {
        Ok({
            // Arrange
            let file = FilePath::access(&"write_string.txt");
            let text = String::from("Hello, World!");

            // Action
            file.write_string(&text)?;

            // Assert
            assert_eq!(file.read_string()?, text);

            // Clean-up
            file.delete()?;
        })
    }

    #[test]
    fn write_lines() -> Result<()> {
        Ok({
            // Arrange
            let file = FilePath::access(&"write_lines.txt");
            let lines = "Hello, World!"
                .split_whitespace()
                .map(ToString::to_string)
                .collect();

            // Action
            file.write_lines(&lines)?;

            // Assert
            assert_eq!(file.read_lines()?, lines);

            // Clean-up
            file.delete()?;
        })
    }

    #[test]
    fn append_string() -> Result<()> {
        Ok({
            // Arrange
            let file = FilePath::access(&"append_string.txt");
            let text = String::from("Hello, World!");
            file.write_string(&text)?;

            // Action
            file.append_string(&text)?;

            // Assert
            assert_eq!(file.read_string()?, format!("{text}{text}"));

            // Clean-up
            file.delete()?;
        })
    }

    #[test]
    fn append_lines() -> Result<()> {
        Ok({
            // Arrange
            let file = FilePath::access(&"append_lines.txt");
            let lines1 = vec!["1", "2"].to_vec_string();
            file.write_lines(&lines1)?;

            // Action
            let lines2 = vec!["3", "4"].to_vec_string();
            file.append_lines(&lines2)?;

            // Assert
            assert_eq!(file.read_lines()?, vec!["1", "2", "3", "4"].to_vec_string());

            // Clean-up
            file.delete()?;
        })
    }

    #[test]
    fn delete() -> Result<()> {
        Ok({
            // Arrange
            let path = "delete.txt";
            let file = FilePath::access(&path);
            mk_file(&path)?;

            // Action
            file.delete()?;

            // Assert
            assert!(!path_of(&path).exists(), "{path} should no longer exist");
        })
    }

    #[test]
    fn copy() -> Result<()> {
        Ok({
            // Arrange
            let from = "copy_from.txt";
            let to = "copy_to.txt";
            let file = FilePath::access(&from);
            file.write_string(&String::from("Hello, World!"))?;

            // Action
            file.copy_to(&to)?;

            // Assert
            assert_eq!(
                super::read_string(&from)?,
                super::read_string(&to)?,
                "{from} and {to} should contain the same text"
            );

            // Clean-up
            super::delete(&from)?;
            super::delete(&to)?;
        })
    }

    #[test]
    fn rename() -> Result<()> {
        Ok({
            // Arrange
            let from = "rename_from.txt";
            let to = "rename_to.txt";
            let text = String::from("Hello, World!");
            let file = FilePath::access(&from);
            file.write_string(&text)?;

            // Action
            file.rename_to(&to)?;

            // Assert
            assert!(!path_of(&from).exists(), "{from} should no longer exist");
            assert_eq!(
                super::read_string(&to)?,
                text,
                "{to} should contain: {text}"
            );

            // Clean-up
            super::delete(&to)?;
        })
    }
}
