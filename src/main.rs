extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn main() {
    if args().len() < 2 || args().len() > 3 {
        eprintln!("Usage : `source` `target`[optional]");
        return;
    }

    let read_file = File::open(args().nth(1).unwrap()).unwrap();

    let mut inp = BufReader::new(read_file);
    let out = if args().len() == 3 {
        File::create(args().nth(2).unwrap()).unwrap()
    } else {
        File::create(format!("{}.gz", args().nth(1).unwrap())).unwrap()
    };

    let mut encoder = GzEncoder::new(out, Compression::default());

    let start = Instant::now();
    copy(&mut inp, &mut encoder).unwrap();
    let out = encoder.finish().unwrap();
    println!(
        "Size(src) : {src_size:?}
        \nSize(out) : {out_size:?}
        \nFinished in {time:?}",
        src_size = inp.get_ref().metadata().unwrap().len(),
        out_size = out.metadata().unwrap().len(),
        time = Instant::elapsed(&start)
    );
}
