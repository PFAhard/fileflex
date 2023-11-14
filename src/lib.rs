use std::{fs::File, path::Path};

type Result<T> = std::io::Result<T>;

pub trait FileFlex: AsRef<Path> {
    fn truncatable(&self) -> File {
        File::options()
            .truncate(true)
            .create(true)
            .open(self)
            .unwrap()
    }

    fn try_truncatable(&self) -> Result<File> {
        File::options().truncate(true).create(true).open(self)
    }

    fn appendable(&self) -> File {
        File::options()
            .append(true)
            .create(true)
            .open(self)
            .unwrap()
    }

    fn try_appendable(&self) -> Result<File> {
        File::options().append(true).create(true).open(self)
    }

    fn readable(&self) -> File {
        File::options().read(true).open(self).unwrap()
    }

    fn try_readable(&self) -> Result<File> {
        File::options().read(true).open(self)
    }
}

impl FileFlex for &str {}

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
