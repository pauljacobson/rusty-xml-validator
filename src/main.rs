use clap::Parser;
mod validator;

/// Command-line arguments structure for the XML validator
/// 
/// Uses clap's derive feature to automatically generate a CLI parser
#[derive(Parser, Debug)]
#[command(name = "wxr_validator")]
#[command(about = "Validate an XML file from a file path or URL")]
struct Args {
    /// Path to the local XML file or URL (http/https)
    source: String,
}

/// Main entry point for the WXR validator application
/// 
/// Parses command-line arguments and runs the validation process
/// on the specified XML file or URL.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command-line arguments
    let args = Args::parse();
    
    // Run the validator on the provided source
    validator::validate_xml(&args.source)?;
    
    Ok(())
}