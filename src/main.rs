use clap::Parser;
mod validator;

#[derive(Parser, Debug)]
#[command(name = "wxr_validator")]
#[command(about = "Validate an XML file from a file path or URL")]
struct Args {
    /// Path to the local XML file or URL (http/https)
    source: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    validator::validate_xml(&args.source)?;
    Ok(())
}