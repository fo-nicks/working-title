mod app;

use std::io::Result;

fn main() -> Result<()> {
    app::init()?;
    Ok(())
}
