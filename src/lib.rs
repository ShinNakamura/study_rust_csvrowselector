use std::io::{self};
use std::collections::HashMap;
use csv;

type MyResult = Result<(), Box<dyn std::error::Error>>;
type RecordMap = HashMap<String, String>;

pub fn run() -> MyResult {
    let input = io::stdin();
    let input = io::BufReader::new(input.lock());
    let mut rdr = csv::Reader::from_reader(input);
    let out = io::stdout();
    let mut wtr = csv::Writer::from_writer(out.lock());
    // ヘッダーの順番を守る
    let header = rdr.headers()?.clone();
    wtr.write_record(&header)?;
    for result in rdr.deserialize() {
        let record_map: RecordMap = result?;
        let mut out: Vec<String> = Vec::new();
        for fld in header.iter() {
            if let Some(val) = record_map.get(fld) {
                out.push(val.to_string());
            }
        }
        wtr.write_record(&out);
    }
    Ok(())
}
