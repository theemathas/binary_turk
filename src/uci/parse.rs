use std::iter::Peekable;
use std::str::FromStr;
use std::time::Duration;

use game::{Position, FromTo, White, Black, fen_to_position, start_pos};
use super::types::{Cmd, RegisterParam, GoParam};
use super::types::options;
use types::{NumNodes, NumPlies, NumMoves};

pub fn parse(s: &str) -> Option<Cmd> {
    let mut words = s.words().peekable();

    let mut cmd_val: Option<Cmd> = None;

    while cmd_val.is_none() {
        let next_word_opt = words.next();
        let next_word = match next_word_opt {
            Some(val) => val,
            None => break,
        };

        cmd_val = match next_word {
            "uci" => Some(Cmd::Uci),
            "debug" => parse_on_off(&mut words).map(|val| Cmd::Debug(val)),
            "isready" => Some(Cmd::IsReady),
            "setoption" => parse_option_val(&mut words)
                           .map(|(name, val)| Cmd::SetOption(name, Some(val))),
            "register" => parse_register_vec(&mut words).map(|val| Cmd::Register(val)),
            "ucinewgame" => Some(Cmd::UciNewGame),
            "position" => {
                let temp = parse_position(&mut words);
                temp.and_then(|pos_opt| {
                    match parse_move_vec(&mut words) {
                        Some(moves) => Some(Cmd::SetupPosition(pos_opt, moves)),
                        None => None,
                    }
                })
            },
            "go" => parse_go_param_vec(&mut words).map(|val| Cmd::Go(val)),
            "stop" => Some(Cmd::Stop),
            "ponderhit" => Some(Cmd::PonderHit),
            "quit" => Some(Cmd::Quit),
            _ => None,
        }
    }

    cmd_val
}

fn parse_on_off(words: &mut Iterator<Item = &str>) -> Option<bool> {
    let mut ans = None;
    while ans.is_none() {
        let next_word_opt = words.next();
        let next_word = match next_word_opt {
            Some(val) => val,
            None => break,
        };
        ans = match next_word {
            "on" => Some(true),
            "off" => Some(false),
            _ => None,
        }
    }
    ans
}

fn parse_option_val<'a,T>(words: &mut T) -> Option<options::NameAndVal>
where T: Iterator<Item = &'a str> {
    // TODO parse options.
    Some((options::Name::Dummy, options::Val::Spin(1)))
}

fn parse_register_vec<'a,T>(words: &mut Peekable<&'a str,T>) -> Option<Vec<RegisterParam>>
where T: Iterator<Item = &'a str> {
    let mut res = Vec::<RegisterParam>::new();
    while let Some(next_word) = words.next() {
        let register_val = match next_word {
            "later" => Some(RegisterParam::Later),
            "name" => {
                let mut name_vec = Vec::<&str>::new();
                loop {
                    let curr_name = words.peek().and_then( |s| {
                        if ["late","name","code"].contains(s) { None }
                        else { Some(*s) }
                    });
                    match curr_name {
                        Some(val) => {
                            name_vec.push(val);
                            words.next();
                        },
                        None => break,
                    }
                }
                Some(RegisterParam::Name(name_vec.connect(" ")))
            },
            "code" => words.next().map( |x| RegisterParam::Code(x.to_string())),
            _ => None,
        };
        if let Some(val) = register_val {
            res.push(val);
        }
    }
    if res.is_empty() { None } else { Some(res) }
}

fn parse_position<'a,T>(words: &mut Peekable<&'a str,T>) -> Option<Position>
where T: Iterator<Item = &'a str> {
    if words.peek() == Some(&"startpos") {
        words.next();
        return Some(start_pos());
    }
    let six_words: Vec<_> = words.by_ref().take(6).collect();
    fen_to_position(&*six_words.connect(" ")).ok()
}

fn parse_move_vec<'a,T>(words: &mut Peekable<&'a str,T>) -> Option<Vec<FromTo>>
where T: Iterator<Item = &'a str> {
    let mut res = Vec::<FromTo>::new();
    while let Some(val) = words.peek().and_then(|val| FromStr::from_str(*val)) {
        res.push(val);
        words.next();
    }
    if res.is_empty() { None } else { Some(res) }
}

fn parse_go_param_vec<'a,T>(words: &mut Peekable<&'a str,T>) -> Option<Vec<GoParam>>
where T: Iterator<Item = &'a str> {
    let mut res = Vec::<GoParam>::new();
    while let Some(next_word) = words.next() {
        match next_word {
            "search moves" => parse_move_vec(words).map(|x| GoParam::SearchMoves(x)),
            "ponder" => Some(GoParam::Ponder),
            "wtime"     => words.next().and_then(|s| s.parse::<i64>())
                                       .map(|x| GoParam::Time(White,Duration::milliseconds(x))),
            "btime"     => words.next().and_then(|s| s.parse::<i64>())
                                       .map(|x| GoParam::Time(Black,Duration::milliseconds(x))),
            "winc"      => words.next().and_then(|s| s.parse::<i64>())
                                       .map(|x| GoParam::IncTime(White,Duration::milliseconds(x))),
            "binc"      => words.next().and_then(|s| s.parse::<i64>())
                                       .map(|x| GoParam::IncTime(Black,Duration::milliseconds(x))),
            "movestogo" => words.next().and_then(|s| s.parse::<u16>())
                                       .map(|x| GoParam::MovesToGo(NumMoves(x))),
            "depth"     => words.next().and_then(|s| s.parse::<u16>())
                                       .map(|x| GoParam::Depth(NumPlies(x))),
            "nodes"     => words.next().and_then(|s| s.parse::<u64>())
                                       .map(|x| GoParam::Nodes(NumNodes(x))),
            "mate"      => words.next().and_then(|s| s.parse::<u16>())
                                       .map(|x| GoParam::Mate(NumMoves(x))),
            "movetime"  => words.next().and_then(|s| s.parse::<i64>())
                                       .map(|x| GoParam::MoveTime(Duration::milliseconds(x))),
            "infinite" => Some(GoParam::Infinite),
            _ => None,
        }.map(|val| res.push(val));
    }
    if res.is_empty() { None } else { Some(res) }
}
