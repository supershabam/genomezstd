use std::io::{stdout, Write};

use bio::io::fasta::Reader;

mod encoding;

fn main() {
    let fasta_file = "files/GCA_000001405.29_GRCh38.p14_genomic.fna";
    let r = Reader::from_file(fasta_file).unwrap();
    let mut records = r.records();
    while let Some(Ok(record)) = records.next() {
        let buf = encoding::encode(record.seq());
        stdout().write_all(buf.as_slice()).expect("while writing to stdout");
    }
}
