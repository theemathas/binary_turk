use std::old_io::LineBufferedWriter;
use std::old_io::stdio::StdWriter;
use std::sync::mpsc::{Receiver, SyncSender};

use search;
use search::Response::BestMove as EngineBestMove;
use search::Response::Report as EngineReport;

use types::Response::{self, BestMove, Info};
use InfoParam::{self, Depth, NodesSearched, PrincipalVariation};

pub fn format_output(mut output: LineBufferedWriter<StdWriter>, rx: Receiver<Response>) {
    for x in rx.iter() {
        writeln!(&mut output, "{}", x).ok().expect("cannot write to output");
        output.flush().ok().expect("cannot flush output");
    }
}

pub fn engine_response_output(rx: Receiver<search::Response>, tx: SyncSender<Response>) {
    for x in rx.iter() {
        match x {
            EngineBestMove(best_move, ponder_move) => {
                tx.send(BestMove(best_move, ponder_move)).unwrap();
            },
            EngineReport(depth, nodes_searched, score, pv) => {
                tx.send(Info(vec![Depth(depth), NodesSearched(nodes_searched)])).unwrap();
                tx.send(Info(vec![InfoParam::Score(None, score),
                                  PrincipalVariation(pv)]
                            )).unwrap();
            },
        }
    }
}
