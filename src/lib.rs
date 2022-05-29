use std::io::{self};
use std::collections::HashMap;
use csv;

type MyResult = Result<(), Box<dyn std::error::Error>>;
type RecordMap = HashMap<String, String>;

pub fn run() -> MyResult {
    let cond_map = get_cond();
    println!("{:#?}", cond_map);
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

fn get_cond() -> Vec<(String, String)> {
    let mut cond_map: Vec<(String, String)> = Vec::new();
    let args = std::env::args();
    if args.len() < 2 {
        return cond_map;
    }
    let mut is_cmd_name = true;
    for arg in args {
        if is_cmd_name {
            is_cmd_name = false;
            continue;
        }
        let s: Vec<&str> = arg.splitn(2, ',').collect();
        if s.len() == 1 {
            cond_map.push((s[0].to_string(), "".to_string()));
        } else {
            cond_map.push((s[0].to_string(), s[1].to_string()));
        }
    }
    cond_map
}
