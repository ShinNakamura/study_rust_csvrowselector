use std::io::{self};

type MyResult = Result<(), Box<dyn std::error::Error>>;

pub fn run() -> MyResult {
    let conds = get_cond();
    let input = io::stdin();
    let input = io::BufReader::new(input.lock());
    let mut rdr = csv::Reader::from_reader(input);
    let stdout = io::stdout();
    let out = io::BufWriter::new(stdout.lock());
    let mut wtr = csv::Writer::from_writer(out);
    let header = rdr.headers()?.clone();
    wtr.write_record(&header)?;
    for result in rdr.records() {
        let record = result?;
        let mut is_match = false;
        for (i, fld) in header.iter().enumerate() {
            if is_match {
                break; // or 検索なので、一個でも条件がマッチしたら成功
            }
            let val = &record[i];
            for (cond_fld, cond_val) in conds.iter() {
                if fld != cond_fld {
                    continue;
                }
                if val != cond_val {
                    continue;
                }
                is_match = true;
            }
        }
        if !is_match {
            continue;
        }
        wtr.write_record(&record)?;
    }
    wtr.flush()?;
    Ok(())
}

fn get_cond() -> Vec<(String, String)> {
    let mut conds: Vec<(String, String)> = Vec::new();
    let args = std::env::args();
    if args.len() < 2 {
        return conds;
    }
    let mut is_cmd_name = true;
    for arg in args {
        if is_cmd_name {
            is_cmd_name = false;
            continue;
        }
        let s: Vec<&str> = arg.splitn(2, ',').collect();
        if s.len() == 1 {
            conds.push((s[0].to_string(), "".to_string()));
        } else {
            conds.push((s[0].to_string(), s[1].to_string()));
        }
    }
    conds
}
