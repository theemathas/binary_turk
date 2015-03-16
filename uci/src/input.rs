use std::io::BufRead;
use std::sync::mpsc::SyncSender;

use types::Cmd;
use parse::parse;

pub fn parse_input<R: BufRead>(input: R, tx: SyncSender<Cmd>) {
    for x in input.lines() {
        let s = x.ok().expect("cannot read input");
        if let Some(cmd) = parse(&*s) {
            tx.send(cmd).ok().expect("parse_input tx is closed");
        }
    }
}
