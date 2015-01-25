use std::io::stdio::StdinReader;
use std::sync::mpsc::SyncSender;

use super::types::Cmd;
use super::parse::parse;

pub fn parse_input(mut input: StdinReader, tx: SyncSender<Cmd>) {
    let mut inbuf = input.lock();
    for x in inbuf.lines() {
        let s = x.ok().expect("cannot read input");
        if let Some(cmd) = parse(&*s) {
            tx.send(cmd).ok().expect("parse_input tx is closed");
        }
    }
}
