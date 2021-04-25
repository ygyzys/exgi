use std::fs::OpenOptions;
use std::io::ErrorKind;

use rusqlite::Connection;

use crate::common::*;

// FIXME: this is currently vulnerable to races
fn init_at(path: &PathBuf, new_only: bool) {
    let db = OpenOptions::new()
        .write(true)
        .truncate(!new_only)
        .create(!new_only)
        .create_new(new_only)
        .open(path);
    if let Err(err) = db {
        let path = path.clone();
        if err.kind() == ErrorKind::AlreadyExists {
            eprintln!("Can't initialize database since it already exists (at `{}`).\n\
                Consider using `exgi reset` command or changing `EXGI_STORAGE_PATH`.", ptos(&path));
        } else {
            eprintln!("Can't create file `{}`: {}", ptos(&path), err);
        }
        std::process::exit(1);
    }
    let conn = Connection::open(path).expect("open db");
    conn.execute("CREATE TABLE book (epd text NOT NULL PRIMARY KEY, \
        white int NOT NULL, black int NOT NULL, \
        eval int, forced int, \
        top_id text, top_rating int);", []).expect("create table");
}

pub fn init() {
    let mut path = get_storage_path();
    path.push(ATOMIC_DB_PATH);
    init_at(&path, true);
}

pub fn reset() {
    let mut path = get_storage_path();
    let mut end_path = path.clone();
    path.push("new.".to_owned() + ATOMIC_DB_PATH);
    end_path.push(ATOMIC_DB_PATH);
    init_at(&path, false);
    if let Err(err) = std::fs::rename(&path, &end_path) {
        eprintln!("Can't move from `{}` to `{}`: {}", ptos(&path), ptos(&end_path), err);
        std::process::exit(1);
    }
}
