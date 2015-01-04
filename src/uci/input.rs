use std::io::stdio::StdinReader;
use std::sync::mpsc::Sender;

use super::types::Cmd;
use super::parse::parse;

pub fn parse_input(mut input: StdinReader, tx: Sender<Cmd>) {
    let mut inbuf = input.lock();
    for x in inbuf.lines() {
        let s = x.unwrap();
        if let Some(cmd) = parse(&*s) {
            let send_res = tx.send(cmd);
            if send_res.is_err() {
                return;
            }
        }
    }
}
