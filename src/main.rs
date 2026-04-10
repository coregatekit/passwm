use passwm::error::Result;

fn main() {
    if let Err(e) = run() {
        eprintln!("❌ Error: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    Ok(())
}
