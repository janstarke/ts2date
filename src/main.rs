use std::io::{self, BufRead, Write};
use regex::Regex;
use chrono::NaiveDateTime;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let re = Regex::new(r"^(?P<lhs>.*?)(?P<ts>\d{10})(?P<rhs>.*)$").unwrap();
    
    for line in stdin.lock().lines() {
        let content = line?;
        let out = match re.captures(&content) {
            Some(caps) => format!("{}{}{}", caps.name("lhs").unwrap().as_str(),
                                            ts2date(caps.name("ts").unwrap().as_str()),
                                            caps.name("rhs").unwrap().as_str()),
            None => content
        };

        io::stdout().write((out+ "\n").as_bytes())?;
    }
    Ok(())
}

fn ts2date(ts: &str) -> String {
    let nt = NaiveDateTime::from_timestamp(ts.parse::<i64>().unwrap(), 0);
    nt.format("%Y-%m-%d %H:%M:%S").to_string()
}