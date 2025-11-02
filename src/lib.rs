#[cfg(feature = "tokio")]
pub mod tokio;

use std::{
    ffi::{OsStr, OsString},
    fmt::Debug,
    fs::File,
    path::{Path, PathBuf},
};

type Result<T> = std::io::Result<T>;

pub trait FileFlex: AsRef<Path> + Debug {
    fn truncatable(&self) -> File {
        match File::options()
            .write(true)
            .truncate(true)
            .create(true)
            .open(self)
        {
            Ok(f) => f,
            Err(err) => {
                dbg!(err, self);
                panic!();
            }
        }
    }

    fn try_truncatable(&self) -> Result<File> {
        File::options().truncate(true).create(true).open(self)
    }

    fn appendable(&self) -> File {
        File::options()
            .write(true)
            .append(true)
            .create(true)
            .open(self)
            .unwrap()
    }

    fn try_appendable(&self) -> Result<File> {
        File::options().append(true).create(true).open(self)
    }

    fn readable(&self) -> File {
        match File::options().read(true).open(self) {
            Ok(f) => f,
            Err(err) => match err.kind() {
                std::io::ErrorKind::NotFound => {
                    dbg!(err, self);
                    panic!();
                }
                _ => todo!(),
            },
        }
    }

    fn try_readable(&self) -> Result<File> {
        File::options().read(true).open(self)
    }
}

impl<F> FileFlex for &F where F: FileFlex {}

impl FileFlex for &str {}

impl FileFlex for PathBuf {}

impl FileFlex for &dyn FileFlex {}

impl FileFlex for Path {}

impl FileFlex for OsString {}

impl FileFlex for OsStr {}

impl FileFlex for String {}

#[test]
fn test_file_flex() {
    use std::io::{Read, Write};

    let mut f = "hello.txt".try_truncatable().unwrap();
    f.write_all(b"world").unwrap();
    f.flush().unwrap();

    let mut f = "hello.txt".try_appendable().unwrap();
    f.write_all(b"world").unwrap();
    f.flush().unwrap();

    let mut f = "hello.txt".try_readable().unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    assert_eq!(s, "world");
}
