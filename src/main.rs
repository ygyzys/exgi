mod common;
mod cli;
mod info;
mod init;
mod top;

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
        eprintln!("init: SUCCESS\nDatabase initialized.");
        return;
    }
    if matches.subcommand_matches("reset").is_some() {
        init::reset();
        eprintln!("reset: SUCCESS\nDatabase reinitialized.");
        return;
    }
    if let Some(submatch) = matches.subcommand_matches("top") {
        top::top(cli::pgn(&submatch), cli::num_players(&submatch));
        eprintln!("top: SUCCESS");
        return;
    }

    if std::env::args().len() == 1 {
        println!("{}", cli::usage());
        std::process::exit(0);
    }
    unreachable!();
}
