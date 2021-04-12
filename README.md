# `exgi`: Atomic opening book

Note: there is no actual book yet.

## Installation

Install stable Rust toolchain and `libsqlite3-dev`.
TODO: instructions for Ubuntu.
```
rustup update
```

Now you can build and install the database:
```
git clone https://github.com/yzygys/exgi
cd exgi
cargo install --path .
exgi
```

If the system cannot find `exgi` binary, most likely you need to add `$HOME/.cargo/bin` to `PATH`. See [cargo install path resolution](https://doc.rust-lang.org/cargo/commands/cargo-install.html). You can also use the database without installing it in the system by running:
```
cargo run --release <args>
```

## Persistent storage

The persistent storage for `exgi` is placed into `$HOME/.exgi` directory by default. If you want to change the storage path, use `EXGI_STORAGE_PATH` environment variable to set another directory. All the paths below are relative with respect to the root of persistent storage.

### `atomic.db`

This is the main database file with all the necessary informations about recorded games and positions. It is an SQLite3 database, and you can read or backup it manually. Manual modifications by SQLite3 toolset are not recommended and can produce incorrect behaviour of `exgi`. TODO: Use `exgi view` for manual modifications (see description below).

## Usage

### Quick start

Use `exgi --help` or `exgi <command-name> --help` to view the built-in help messages.
TODO: Show how to construct a simple database and view it.

### `exgi init`

Initializes an empty database: `atomic.db` file with the appropriate schema for the current release of `exgi`. If the database already exists, this command fails.

### `exgi reset`

Equivalent to `exgi init`, but if the database exists, it deletes the existing database and creates a new one, tabula rasa.

## License

`exgi` is licensed under the AGPLv3+. See `LICENSE.txt` for the full license text.
