use std::io::LineBufferedWriter;
use std::io::stdio::StdWriter;
use std::sync::mpsc::Receiver;

use super::types::Response;

pub fn format_output(mut output: LineBufferedWriter<StdWriter>, rx: Receiver<Response>) {
    for x in rx.iter() {
        writeln!(&mut output, "{}", x).unwrap();
        output.flush().unwrap();
    }
}
