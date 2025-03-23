use quick_xml::Reader;
use quick_xml::events::Event;
use reqwest::blocking::get;
use std::fs::File;
use std::io::{BufReader, BufRead, Cursor};

pub fn validate_xml(source: &str) -> Result<(), Box<dyn std::error::Error>> {
    let input = load_input(source)?;
    let mut reader = Reader::from_reader(input);
    reader.trim_text(true);

    let mut buf = Vec::new();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Text(e)) => {
                let text = e.unescape_and_decode(&reader)?;
                if contains_control_chars(&text) {
                    println!("Control character found: {}", text);
                }
            }
            Ok(Event::CData(e)) => {
                let text = e.unescape_and_decode(&reader)?;
                if contains_control_chars(&text) {
                    println!("Control character found in CDATA: {}", text);
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                println!("Error at position {}: {:?}", reader.buffer_position(), e);
                break;
            }
            _ => (),
        }
        buf.clear();
    }
    Ok(())
}

fn contains_control_chars(s: &str) -> bool {
    s.chars().any(|c| c.is_control() && !c.is_whitespace())
}

fn load_input(source: &str) -> Result<Box<dyn BufRead>, Box<dyn std::error::Error>> {
    if source.starts_with("http://") || source.starts_with("https://") {
        let response = get(source)?;
        if !response.status().is_success() {
            return Err(format!("HTTP error: {}", response.status()).into());
        }
        let text = response.text()?;
        Ok(Box::new(Cursor::new(text)))
    } else {
        let file = File::open(source)?;
        Ok(Box::new(BufReader::new(file)))
    }
}