use crate::common::*;

#[derive(Default)]
pub struct Player {
    pub nickname: String,
    pub rating: i16,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", &self.nickname, self.rating)
    }
}

#[derive(Default)]
pub struct InfoPass {
    // current game info
    players: [Player; 2],
    // accumulated info
    nickname_to_top_rating: HashMap<String, i16>,
}

impl InfoPass {
    // only call once
    pub fn top(&mut self, num: Option<unz>) -> Vec<Player> {
        let mut players: Vec<Player> = std::mem::take(&mut self.nickname_to_top_rating)
            .into_iter().map(|(n, e)| Player{nickname: n, rating: e}).collect();
        players.sort_by(|a, b| b.rating.cmp(&a.rating));
        if let Some(n) = num {
            players.truncate(n.get());
        }
        players
    }
}

fn get_rating(value: &[u8]) -> Result<i16, btoi::ParseIntegerError> {
    if value == b"?" { Ok(0) } else { btoi::btoi(value) }
}

fn get_nickname(value: &[u8]) -> String {
    String::from_utf8_lossy(value).to_string()
}

impl Visitor for InfoPass {
    type Result = ();

    fn header(&mut self, key: &[u8], raw: RawHeader) {
        let value = raw.as_bytes();
        if key == b"WhiteElo" {
            self.players[WHITE].rating = get_rating(value).expect("WhiteElo");
        } else if key == b"BlackElo" {
            self.players[BLACK].rating = get_rating(value).expect("BlackElo");
        } else if key == b"White" {
            self.players[WHITE].nickname = get_nickname(value);
        } else if key == b"Black" {
            self.players[BLACK].nickname = get_nickname(value);
        }
    }

    fn end_headers(&mut self) -> Skip {
        for &color in &[WHITE, BLACK] {
            let nickname = std::mem::take(&mut self.players[color].nickname);
            let rating = std::mem::take(&mut self.players[color].rating);
            if nickname.is_empty() {
                eprintln!("Warning: no username!");
                continue;
            }
            let entry = self.nickname_to_top_rating.entry(nickname).or_insert(rating);
            if *entry < rating {
                *entry = rating;
            }
        }
        Skip(true)
    }

    fn end_game(&mut self) {}
}
