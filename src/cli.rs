use clap::{App, ArgMatches, clap_app, crate_version, Shell};
use clap::AppSettings::{DisableHelpSubcommand, VersionlessSubcommands};
use madvise::{AccessPattern, AdviseMemory};

use crate::common::*;

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
            (@arg PGN: -p --pgn +takes_value "input .pgn or .pgn.bz2 file")
            (@arg MONTH: -m --month +takes_value "month in YYYY-MM format to use the corresponding .pgn.bz2 file from lichess game database")
            (@arg download: -d --download "use with --month option to download the file if it is not yet present in cache")
            (@arg mmap: --mmap "use mmap for reading pgn")
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

pub fn num_players(sub: &ArgMatches) -> Option<unz> {
    let num = sub.value_of_lossy("NUM");
    if num.is_none() {
        return None;
    }
    let num = num.unwrap();
    let res = num.parse::<usize>();
    if let Err(err) = res {
        eprintln!("Can't parse number of players `{}`: {}", num, err);
        std::process::exit(1);
    }
    let retval = unz::new(res.unwrap());
    if retval.is_none() {
        eprintln!("Number of players must be positive.");
        std::process::exit(1);
    }
    retval
}

fn pgn_file(path: &Path) -> File {
    let file = File::open(path);
    if let Err(err) = file {
        eprintln!("PGN file `{}` can't be read: {}", ptos(&path), err);
        std::process::exit(1);
    }
    file.unwrap()
}

fn pgn_bz2(path: &Path) -> PGNSeeker {
    PGNSeeker::Bzip2(pgn_file(path))
}

fn raw_pgn(path: &Path, mmapped: bool) -> PGNSeeker {
    let file = pgn_file(path);
    if mmapped {
        let pgn_mmap = unsafe { Mmap::map(&file).expect("mmap") };
        pgn_mmap.advise_memory_access(AccessPattern::Sequential).expect("madvise");
        PGNSeeker::Mmapped(pgn_mmap)
    } else {
        PGNSeeker::Raw(file)
    }
}

fn cached_pgn(month: &str, download: bool) -> PGNSeeker {
    /*// let path = get path
    if download && path_not_exists {
        eprintln!("downloading {}", path);
        // download pgn
    }
    pgn_bz2(path)*/
    unimplemented!()
}

pub fn pgn(sub: &ArgMatches) -> PGNSeeker {
    let pgn = sub.value_of_os("PGN");
    let month = sub.value_of_lossy("MONTH");
    if pgn.is_none() && month.is_none() {
        eprintln!("PGN must be provided.");
        std::process::exit(1);
    }
    if pgn.is_some() && month.is_some() {
        eprintln!("Either PGN path or month must be provided, not both.");
        std::process::exit(1);
    }

    let download = sub.is_present("download");
    let mmapped = sub.is_present("mmap");
    if let Some(path) = pgn {
        if download {
            eprintln!("Warning: download option is ignored since PGN is provided by path.");
        }
        let path = Path::new(path);
        return if path.ends_with(".bz2") { pgn_bz2(path) }
            else { raw_pgn(path, mmapped) };
    }
    if let Some(date) = month {
        return cached_pgn(&date, download);
    }
    unreachable!()
}
