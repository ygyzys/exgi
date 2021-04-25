pub use std::collections::{HashMap, HashSet};
pub use std::fmt;
pub use std::fs::File;
pub use std::num::{NonZeroUsize as unz};
pub use std::path::{Path, PathBuf};

pub use memmap::Mmap;
pub use pgn_reader::{BufferedReader, Skip, RawHeader, Visitor};

pub const ATOMIC_DB_PATH: &'static str = "atomic.db";
pub const WHITE: usize = 0;
pub const BLACK: usize = 1;

pub fn ptos<T: AsRef<Path>>(path: &T) -> &str {
    path.as_ref().to_str().expect("os string")
}

pub fn get_storage_path() -> PathBuf {
    let path: PathBuf = std::env::var("EXGI_STORAGE_PATH")
        .map(|s| s.into())
        .unwrap_or_else(|_| {
            let mut p = dirs::home_dir().expect("home");
            p.push(".exgi");
            p
        });
    if let Err(err) = std::fs::create_dir_all(&path) {
        eprintln!("Storage directory `{}` can't be accessed or created: {}", ptos(&path), err);
        std::process::exit(1);
    }
    path
}

pub enum PGNSeeker {
    Mmapped(Mmap),
    Raw(File),
    Bzip2(File),
}

impl PGNSeeker {
    pub fn read_all<V: Visitor>(&self, v: &mut V) -> std::io::Result<()> {
        use PGNSeeker::*;
        match self {
            Mmapped(r) => BufferedReader::new(&r[..]).read_all(v),
            Raw(f) => BufferedReader::new(f.try_clone().expect("try_clone")).read_all(v),
            Bzip2(f) => BufferedReader::new(bzip2_rs::DecoderReader::new(
                f.try_clone().expect("try_clone"))).read_all(v),
        }
    }
}
