use std::{
    ffi::{OsStr, OsString},
    fmt::Debug,
    path::{Path, PathBuf},
};

use tokio::fs::File;

type Result<T> = std::io::Result<T>;

pub trait FileFlexAsync: AsRef<Path> + Debug
where
    Self: Send + Sync,
{
    fn tokio_truncatable(&self) -> impl std::future::Future<Output = File> + Send {
        async move {
            match File::options()
                .write(true)
                .truncate(true)
                .create(true)
                .open(self)
                .await
            {
                Ok(f) => f,
                Err(err) => {
                    dbg!(err, self);
                    panic!();
                }
            }
        }
    }

    fn tokio_try_truncatable(&self) -> impl std::future::Future<Output = Result<File>> + Send {
        async move { File::options().truncate(true).create(true).open(self).await }
    }

    fn tokio_appendable(&self) -> impl std::future::Future<Output = File> + Send {
        async move {
            File::options()
                .write(true)
                .append(true)
                .create(true)
                .open(self)
                .await
                .unwrap()
        }
    }

    fn tokio_try_appendable(&self) -> impl std::future::Future<Output = Result<File>> + Send {
        async move { File::options().append(true).create(true).open(self).await }
    }

    fn tokio_readable(&self) -> impl std::future::Future<Output = File> + Send {
        async move {
            match File::options().read(true).open(self).await {
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
    }

    fn tokio_try_readable(&self) -> impl std::future::Future<Output = Result<File>> + Send {
        async move { File::options().read(true).open(self).await }
    }
}

impl<F> FileFlexAsync for &F where F: FileFlexAsync {}

impl FileFlexAsync for &str {}

impl FileFlexAsync for PathBuf {}

impl FileFlexAsync for Path {}

impl FileFlexAsync for OsString {}

impl FileFlexAsync for OsStr {}

impl FileFlexAsync for String {}
