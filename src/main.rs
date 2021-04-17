mod common;
mod cli;
mod init;

fn main() {
    let matches = cli::app().get_matches_safe();

    if let Err(ref err) = matches {
        let usage_idx = err.message.find("\nUSAGE:");
        if !err.message.contains(27 as char) || usage_idx.is_none() {
            // handle --help and --version options
            // which are for whatever reason considered as error match by clap
            println!("{}", err.message);
            std::process::exit(0);
        } else {
            // remove short USAGE and replace it with adequate one
            println!("{}\n\n{}", cli::usage(), &err.message[..usage_idx.unwrap()]);
            std::process::exit(1);
        }
    }

    let matches = matches.unwrap();
    if matches.subcommand_matches("_bash").is_some() {
        cli::complete(clap::Shell::Bash);
        return;
    }
    if matches.subcommand_matches("_zsh").is_some() {
        cli::complete(clap::Shell::Zsh);
        return;
    }
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
        println!("{}", cli::usage());
        std::process::exit(0);
    }
    unreachable!();
}
