use crate::common::*;
use crate::info::InfoPass;

pub fn top(pgn: PGNSeeker, num_players: Option<unz>) {
    let mut pgn_info = InfoPass::default();
    pgn.read_all(&mut pgn_info).expect("read_all");
    for player in pgn_info.top(num_players).into_iter() {
        println!("{}", player);
    }
}
