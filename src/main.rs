mod common;
mod init;

use clap::{App, clap_app, crate_version};
use clap::AppSettings::{DisableHelpSubcommand, VersionlessSubcommands};

fn usage(app: &mut App) -> String {
    let mut help_buf: Vec<u8> = Vec::new();
    app.write_long_help(&mut help_buf).expect("write_help");
    String::from_utf8(help_buf).expect("utf8")
}

// Add autocompletion using clap
fn main() {
    let mut app = clap_app!(exgi =>
        (version: crate_version!())
        (about: "atomic opening book")
        (setting: VersionlessSubcommands)
        (setting: DisableHelpSubcommand)
        (@subcommand init =>
            (about: "initialize database the first time")
        )
        (@subcommand reset =>
            (about: "delete existing database and initialize again")
        )
    );
    let help_msg = usage(&mut app);
    let matches = app.get_matches_safe();

    if let Err(ref err) = matches {
        let usage_idx = err.message.find("USAGE:");
        if !err.message.contains(27 as char) || usage_idx.is_none() {
            // handle --help and --version options
            // which are for whatever reason considered as error match by clap
            println!("{}", err.message);
            std::process::exit(0);
        } else {
            // remove short USAGE and replaces it with adequate one
            println!("{}{}", &err.message[..usage_idx.unwrap()], &help_msg);
            std::process::exit(1);
        }
    }

    let matches = matches.unwrap();
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

    if std::env::args().len() == 1 {
        println!("{}", help_msg);
        std::process::exit(0);
    }
    unreachable!();
}
