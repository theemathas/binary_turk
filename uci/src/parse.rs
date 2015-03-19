use std::iter::Peekable;
use std::str::FromStr;
use std::time::Duration;

use game::{Position, FromTo, White, Black, NumPlies, NumMoves};
use search::NumNodes;
use types::{Cmd, RegisterParam, GoParam};
use types::options;

pub fn parse(s: &str) -> Option<Cmd> {
    let mut words = s.split(' ');

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
                               .map(|x| Cmd::SetOption(x)),
            "register" => parse_register_vec(&mut words).map(|val| Cmd::Register(val)),
            "ucinewgame" => Some(Cmd::UciNewGame),
            "position" => parse_setup_position(&mut words)
                              .map(|(pos, moves)| Cmd::SetupPosition(pos, moves)),
            "go" => parse_go_param_vec(&mut words).map(|val| Cmd::Go(val)),
            "stop" => Some(Cmd::Stop),
            "ponderhit" => Some(Cmd::PonderHit),
            "quit" => Some(Cmd::Quit),
            _ => None,
        }
    }

    if let Some(Cmd::Debug(val)) = cmd_val {
        // TODO set debug
        debug!("debug is now {:?}", val);
    }

    debug!("parse() returning {:?}", cmd_val);

    cmd_val
}

fn parse_on_off<'a, I>(mut words: I) -> Option<bool>
where I: Iterator<Item = &'a str>{
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

    debug!("parse_on_off() returning {:?}", ans);

    ans
}

fn parse_option_val<'a, I>(_words: I) -> Option<options::NameAndVal>
where I: Iterator<Item = &'a str> {
    debug!("dummy option parsing");
    // TODO parse options.
    Some(options::NameAndVal::Dummy(1))
}

fn parse_register_vec<'a, I>(words: I) -> Option<Vec<RegisterParam>>
where I: Iterator<Item = &'a str> {
    let mut words = words.peekable();
    let mut res = Vec::<RegisterParam>::new();
    while let Some(next_word) = words.next() {
        let register_val = match next_word {
            "later" => Some(RegisterParam::Later),
            "name" => {
                let mut name_vec = Vec::<&str>::new();
                loop {
                    let curr_name = words.peek().and_then( |s| {
                        if ["later", "name", "code"].contains(s) { None }
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
    let ans = if res.is_empty() { None } else { Some(res) };

    debug!("parse_register_vec() returning {:?}", ans);

    ans
}

fn parse_setup_position<'a, I>(mut words: I) -> Option<(Position, Vec<FromTo>)>
where I: Iterator<Item = &'a str> {
    let temp = parse_position(words.by_ref());
    temp.map(|pos| {
        // consume everything up to and including "moves"
        let mut words = words.skip_while(|&s| s != "moves");
        words.next();

        // Attempt to parse the moves
        let moves = match parse_from_to_vec(&mut words.peekable()) {
            Some(val) => val,
            None => Vec::new(),
        };

        (pos, moves)
    })
}

fn parse_position<'a, I>(words: I) -> Option<Position>
where I: Iterator<Item = &'a str> {
    let mut words = words.peekable();
    let ans = if words.peek() == Some(&"startpos") {
        debug!("parse_position() consumed \"startpos\"");
        words.next();
        Some(Position::start())
    } else if words.peek() == Some(&"fen") {
        words.next();
        let six_words: Vec<_> = words.by_ref().take(6).collect();
        debug!("parse_position(): six_words = {:?}", six_words);
        six_words.connect(" ").parse::<Position>().ok()
    } else {
        None
    };

    debug!("parse_position() returning {:?}", ans);

    ans
}

fn parse_from_to_vec<'a, I>(words: &mut Peekable<I>) -> Option<Vec<FromTo>>
where I: Iterator<Item = &'a str> {
    let mut res = Vec::<FromTo>::new();
    debug!("parse_from_to_vec() peeked at {:?}", words.peek());
    while let Some(val) = words.peek().and_then(|val| FromStr::from_str(*val).ok()) {
        debug!("parse_from_to_vec(): val = {:?}", val);
        res.push(val);
        words.next();
        debug!("parse_from_to_vec() peeked at {:?}", words.peek());
    }
    let ans = if res.is_empty() { None } else { Some(res) };

    debug!("parse_from_to_vec() returning {:?}", ans);

    ans
}

fn parse_go_param_vec<'a, I>(words: I) -> Option<Vec<GoParam>>
where I: Iterator<Item = &'a str> {
    let mut words = words.peekable();
    let mut res = Vec::<GoParam>::new();
    while let Some(next_word) = words.next() {
        match next_word {
            "searchmoves" => parse_from_to_vec(&mut words).map(|x| GoParam::SearchMoves(x)),
            "ponder" => Some(GoParam::Ponder),
            "wtime"     => words.next().and_then(|s| s.parse::<i64>().ok())
                                       .map(|x| GoParam::Time(White,
                                                              Duration::milliseconds(x))),
            "btime"     => words.next().and_then(|s| s.parse::<i64>().ok())
                                       .map(|x| GoParam::Time(Black,
                                                              Duration::milliseconds(x))),
            "winc"      => words.next().and_then(|s| s.parse::<i64>().ok())
                                       .map(|x| GoParam::IncTime(White,
                                                                 Duration::milliseconds(x))),
            "binc"      => words.next().and_then(|s| s.parse::<i64>().ok())
                                       .map(|x| GoParam::IncTime(Black,
                                                                 Duration::milliseconds(x))),
            "movestogo" => words.next().and_then(|s| s.parse::<u32>().ok())
                                       .map(|x| GoParam::MovesToGo(NumMoves(x))),
            "depth"     => words.next().and_then(|s| s.parse::<u32>().ok())
                                       .map(|x| GoParam::Depth(NumPlies(x))),
            "nodes"     => words.next().and_then(|s| s.parse::<u64>().ok())
                                       .map(|x| GoParam::Nodes(NumNodes(x))),
            "mate"      => words.next().and_then(|s| s.parse::<u32>().ok())
                                       .map(|x| GoParam::Mate(NumMoves(x))),
            "movetime"  => words.next().and_then(|s| s.parse::<i64>().ok())
                                       .map(|x| GoParam::MoveTime(Duration::milliseconds(x))),
            "infinite" => Some(GoParam::Infinite),
            _ => None,
        }.map(|val| res.push(val));
    }
    let ans = if res.is_empty() { None } else { Some(res) };

    debug!("parse_go_param_vec() returning {:?}", ans);

    ans
}
