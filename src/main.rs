mod common;
mod init;

use clap::{clap_app, crate_version, AppSettings::DisableVersion};

// Add autocompletion using clap
fn main() {
    let mut app = clap_app!(exgi =>
        (version: crate_version!())
        (about: "atomic opening book")
        (@subcommand init =>
            (about: "initialize database the first time")
            (setting: DisableVersion)
        )
        (@subcommand reset =>
            (about: "delete existing database and initialize again")
            (setting: DisableVersion)
        )
    );
    let mut help_buf: Vec<u8> = Vec::new();
    app.write_long_help(&mut help_buf).expect("write_help");
    let matches = app.get_matches();
    if matches.subcommand_matches("init").is_some() {
        init::init();
        println!("Database initialized successfully.");
        return;
    }
    if matches.subcommand_matches("reset").is_some() {
        init::reset();
        println!("Database reset successfully.");
        return;
    }
    println!("{}", String::from_utf8(help_buf).unwrap());
}
