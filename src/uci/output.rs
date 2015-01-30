use std::old_io::LineBufferedWriter;
use std::old_io::stdio::StdWriter;
use std::sync::mpsc::Receiver;

use super::types::Response;

pub fn format_output(mut output: LineBufferedWriter<StdWriter>, rx: Receiver<Response>) {
    for x in rx.iter() {
        writeln!(&mut output, "{}", x).ok().expect("cannot write to output");
        output.flush().ok().expect("cannot flush output");
    }
}
