use quick_xml::Reader;
use quick_xml::events::Event;
use reqwest::blocking::get;
use std::fs::File;
use std::io::{BufReader, BufRead, Cursor};

/// Validates an XML file by checking for control characters in text content
/// 
/// # Arguments
/// * `source` - A string representing either a file path or URL to the XML content
/// 
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - Ok(()) if validation succeeds, or an error
pub fn validate_xml(source: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Load the input from either file or URL as a buffered reader
    let input = load_input(source)?;
    
    // Create an XML reader and configure it to trim whitespace
    let mut reader = Reader::from_reader(input);
    reader.trim_text(true);

    // Buffer to hold XML event data
    let mut buf = Vec::new();
    
    // Process XML events until EOF or error
    loop {
        match reader.read_event(&mut buf) {
            // Check regular text content for control characters
            Ok(Event::Text(e)) => {
                let text = e.unescape_and_decode(&reader)?;
                if contains_control_chars(&text) {
                    println!("Control character found: {}", text);
                }
            }
            // Check CDATA sections for control characters
            Ok(Event::CData(e)) => {
                let text = e.unescape_and_decode(&reader)?;
                if contains_control_chars(&text) {
                    println!("Control character found in CDATA: {}", text);
                }
            }
            // Exit loop when reaching end of file
            Ok(Event::Eof) => break,
            // Handle XML parsing errors
            Err(e) => {
                println!("Error at position {}: {:?}", reader.buffer_position(), e);
                break;
            }
            // Ignore all other XML event types (tags, comments, etc.)
            _ => (),
        }
        // Clear the buffer for the next event
        buf.clear();
    }
    Ok(())
}

/// Checks if a string contains any control characters (excluding whitespace)
/// 
/// Control characters are non-printing characters that can cause issues when 
/// importing WordPress content. This function ignores whitespace control chars
/// like newlines, tabs, etc.
/// 
/// # Arguments
/// * `s` - The string to check
/// 
/// # Returns
/// * `bool` - true if control characters are found, false otherwise
fn contains_control_chars(s: &str) -> bool {
    s.chars().any(|c| c.is_control() && !c.is_whitespace())
}

/// Loads input content from either a file path or URL
/// 
/// Handles both local file paths and remote URLs (http/https),
/// returning a boxed BufRead implementation that can be used with quick-xml.
/// 
/// # Arguments
/// * `source` - A string representing either a file path or URL
/// 
/// # Returns
/// * `Result<Box<dyn BufRead>, Box<dyn std::error::Error>>` - A boxed BufRead implementation
fn load_input(source: &str) -> Result<Box<dyn BufRead>, Box<dyn std::error::Error>> {
    // Handle URLs (http or https)
    if source.starts_with("http://") || source.starts_with("https://") {
        let response = get(source)?;
        if !response.status().is_success() {
            return Err(format!("HTTP error: {}", response.status()).into());
        }
        let text = response.text()?;
        Ok(Box::new(Cursor::new(text)))
    } else {
        // Handle local files
        let file = File::open(source)?;
        Ok(Box::new(BufReader::new(file)))
    }
}