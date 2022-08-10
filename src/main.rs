use anyhow::Result;
use log::Level;

#[async_std::main]
async fn main() -> Result<()> {
    simple_logger::init_with_level(Level::Info).unwrap();
    
    // TODO

    Ok(())
}
