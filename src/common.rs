use std::path::PathBuf;

pub const ATOMIC_DB_PATH: &'static str = "atomic.db";

pub fn ptos(path: PathBuf) -> String {
    path.into_os_string().into_string().expect("os string")
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
        println!("Storage directory `{}` can't be accessed or created: {}", ptos(path), err);
        std::process::exit(1);
    }
    path
}
