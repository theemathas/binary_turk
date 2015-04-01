use std::io::{Write, LineWriter};
use std::sync::mpsc::{Receiver, SyncSender};

use search;

use types::Response::{self, Info};
use InfoParam::{self, Depth, NodesSearched, PrincipalVariation};

pub fn format_output<W: Write>(output: W, rx: Receiver<Response>) {
    let mut output = LineWriter::new(output);
    for x in rx.iter() {
        writeln!(&mut output, "{}", x).ok().expect("cannot write to output");
    }
}

pub fn engine_response_output(rx: Receiver<search::Report>, tx: SyncSender<Response>) {
    for search::Report { data, score, pv } in rx.iter() {
        tx.send(Info(vec![Depth(data.depth), NodesSearched(data.nodes)])).unwrap();
        tx.send(Info(vec![InfoParam::Score(None, score),
                          PrincipalVariation(pv)]
                    )).unwrap();

    }
}
