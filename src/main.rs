use protobuf::Message;
use std::fs::File;
use std::io::{BufReader, Result};

fn main() -> Result<()> {
    let sample_verb = File::open("ababillarse.dat")?;
    let mut reader = BufReader::new(sample_verb);

    let mut proto_buff = ::protobuf::CodedInputStream::from_buffered_reader(&mut reader);
    let mut verb = protocol::verb::Verb::new();
    let merged = verb.merge_from(&mut proto_buff);

    match merged {
        Ok(_) => println!("{:?}", verb.take_description()),
        Err(e) => println!("{:?}", e),
    }

    Ok(())
}
