use clap::{App, clap_app, crate_version, Shell};
use clap::AppSettings::{DisableHelpSubcommand, VersionlessSubcommands};

pub fn app() -> App<'static, 'static> {
    clap_app!(exgi =>
        (version: crate_version!())
        (about: "atomic opening book")
        (setting: VersionlessSubcommands)
        (setting: DisableHelpSubcommand)
        (@subcommand _bash =>
            (about: "print bash completion script for exgi")
        )
        (@subcommand _zsh =>
            (about: "print zsh completion script for exgi")
        )
        (@subcommand init =>
            (about: "initialize database the first time")
        )
        (@subcommand reset =>
            (about: "delete existing database and initialize again")
        )
        (@subcommand top =>
            (about: "list top players in the pgn file according to rating maximum reached")
            (@arg NUM: -n --num_players +takes_value "number of top players to print")
            (@arg PATH: -p --pgn +takes_value "input .pgn or .pgn.bz2 file")
            (@arg MONTH: -m --month +takes_value "month in YYYY-MM format to use the corresponding .pgn.bz2 file from lichess game database")
            (@arg download: -d --download "use with --month option to download the file if it is not yet present in cache")
        )
    )
}

pub fn usage() -> String {
    let mut buf: Vec<u8> = Vec::new();
    app().write_long_help(&mut buf).expect("write_help");
    String::from_utf8(buf).expect("utf-8")
}

pub fn complete(shell: Shell) {
    app().gen_completions_to("exgi", shell, &mut std::io::stdout());
}
