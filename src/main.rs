extern crate flate2;

use flate2::write::GzDecoder;
use std::io::BufReader;
use std::io::copy;
use std::fs::File;
use std::env::args;
use std::time::Instant;

fn main() {
    if args().len() < 3 {
        eprintln!("Insufficient Arguments\nUsage: 'source' 'target'");
        return;
    }
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();
    let mut decoder = GzDecoder::new(output);
    let start = Instant::now();
    copy(&mut input,&mut decoder).unwrap();
    let output = decoder.finish().unwrap();
    println!("Source len:{:?}",input.get_ref().metadata().unwrap().len());
    println!("Target len:{:?}",output.metadata().unwrap().len());
    println!("Elapsed: {:?}",start.elapsed());
}
