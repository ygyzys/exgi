# exgi: Atomic opening book

Note: there is no actual book yet.

## Installation

### Platform

This project should easily build on every major platform according to instructions below. However, since it is a CLI application, for Windows users it is recommended to install it on [WSL](https://docs.microsoft.com/en-us/windows/wsl/install-win10).

### Dependencies

Git, GCC, stable Rust and Cargo should be installed on your system. Check if `git`, `cargo` and `cc` commands work in terminal. See the installation instructions in the official documentation for [Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git) and [Rust](https://www.rust-lang.org/tools/install) respectively if any of them doesn't work. On Mac the linker `cc` should always be preinstalled, and for Linux and WSL see this [troubleshooting tip](https://ostechnix.com/how-to-fix-rust-error-linker-cc-not-found-on-linux/). In this project it is needed to link Rust code to [Sqlite3](https://www.sqlite.org), which is written in C.

### Build and install

Installing the binary is as simple as:
```
git clone https://github.com/ygyzys/exgi
cd exgi
cargo install --path .
```

Now you should be able to run `exgi` from console. If the system cannot find `exgi` binary, most likely you need to add `$HOME/.cargo/bin` to `PATH`. See [cargo install path resolution](https://doc.rust-lang.org/cargo/commands/cargo-install.html). You can also use `exgi` without installing it in the system by running this from the root of this repository:
```
cargo run --release <args>
```

### Shell completion

If you want Bash to autocomplete arguments for `exgi` (as in `exgi re<Tab>` should expand into `exgi reset`), you can just run `./install.sh`. If you use Zsh rather than Bash, run `exgi _zsh > <directory>/_exgi`, where `<directory>` is present in `$FPATH`.

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
